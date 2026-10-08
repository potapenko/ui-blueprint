use uiblueprint_schema::{SchemaVersion, model::*};
fn response() -> Document {
    let doc = Document::from_json(
        include_bytes!("../../../fixtures/golden/GRAPH-MANY-TO-MANY.json"),
        1_000_000,
    )
    .unwrap();
    let Artifact::Snapshot(mut s) = doc.artifact else {
        panic!("mixed source graph")
    };
    s.context.projection = Projection::Design;
    Document {
        schema_version: SchemaVersion::CURRENT,
        artifact: Artifact::ChannelResponse(Box::new(ChannelResponse {
            request_id: Id("explicit-request".into()),
            session_id: s.context.session_id.clone(),
            dispatch_sequence: 1,
            target: s.context.target.clone(),
            channel: Channel::OptInLayoutProbe,
            result: ChannelResult::Observed(s),
        })),
    }
}
fn parts(d: &mut Document) -> (&mut Channel, &mut Snapshot) {
    let Artifact::ChannelResponse(r) = &mut d.artifact else {
        panic!()
    };
    let ChannelResult::Observed(s) = &mut r.result else {
        panic!()
    };
    (&mut r.channel, s)
}
#[test]
fn only_exact_ax_probe_design_composition_joins_unchanged_homogeneous_cases() {
    let d = response();
    d.validate().unwrap();
    let bytes = serde_json::to_vec(&d).unwrap();
    assert_eq!(Document::from_json(&bytes, bytes.len()).unwrap(), d);
    for channel in [
        Channel::ExternalSemantics,
        Channel::RenderedCapture,
        Channel::OptInLayoutProbe,
    ] {
        let mut d = response();
        let (wrapper, s) = parts(&mut d);
        *wrapper = channel;
        for o in &mut s.observations {
            o.channel = channel;
        }
        d.validate().unwrap();
    }
    for variant in 0..5 {
        let mut d = response();
        let (wrapper, s) = parts(&mut d);
        match variant {
            0 => s.context.projection = Projection::Interaction,
            1 => *wrapper = Channel::ExternalSemantics,
            2 => {
                for o in &mut s.observations {
                    o.channel = Channel::ExternalSemantics;
                }
            }
            3 => s.observations[0].channel = Channel::RenderedCapture,
            _ => {
                let mut o = s.observations[0].clone();
                o.id = Id("third-channel".into());
                o.channel = Channel::RenderedCapture;
                s.observations.push(o);
            }
        }
        assert!(d.validate().is_err(), "variant{variant}");
    }
}
#[test]
fn composition_does_not_relax_source_binding_evidence_or_privacy() {
    for variant in 0..4 {
        let mut d = response();
        let (_, s) = parts(&mut d);
        match variant {
            0 => s.context.target.generation = Id("wrong-generation".into()),
            1 => s.context.session_id = Id("wrong-session".into()),
            2 => {
                if let Property::Requested { evidence, .. } = &mut s.nodes[0].properties[0] {
                    evidence.observation_id = Id("absent".into());
                }
            }
            _ => {
                if let Property::Requested { sensitivity, .. } = &mut s.nodes[0].properties[0] {
                    *sensitivity = Sensitivity::Sensitive;
                }
            }
        }
        assert!(d.validate().is_err(), "variant{variant}");
    }
}
