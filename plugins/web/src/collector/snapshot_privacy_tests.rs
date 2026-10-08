// Q01's independent two-node native-table counterexample, extended to isolate
// each source field and retain safe URL facts. No browser or publication owner.
use super::*;
#[test]
fn source_url_facts_refuse_before_canonical_storage_and_preserve_safe_values() {
    let doc = Document::from_json(
        include_bytes!("../../../../fixtures/golden/ENV-REQUEST-VALID.json"),
        65536,
    )
    .unwrap();
    let Artifact::Request(mut r) = doc.artifact else {
        panic!()
    };
    r.context.fields = vec![Field::Value];
    let surface = r.context.surfaces[0].clone();
    let empty = serde_json::json!({"index":[],"value":[]});
    let flags = serde_json::json!({"index":[]});
    let mut raw = serde_json::json!({"strings":["#document","IMG","srcset","/image?token=Q01_SYNTHETIC_URL_CANARY 1x",surface.id.0,"http://127.0.0.1/fixture","http://127.0.0.1/image?token=Q01_SYNTHETIC_URL_CANARY"],"documents":[{
   "documentURL":5,"title":-1,"baseURL":5,"contentLanguage":-1,"encodingName":-1,"publicId":-1,"systemId":-1,"frameId":4,
   "nodes":{"parentIndex":[-1,0],"nodeType":[9,1],"nodeName":[0,1],"nodeValue":[-1,-1],"backendNodeId":[7,11],"attributes":[[],[2,3]],"shadowRootType":empty,"textValue":empty,"inputValue":empty,"inputChecked":flags,"optionSelected":flags,"contentDocumentIndex":empty,"pseudoType":empty,"pseudoIdentifier":empty,"isClickable":flags,"currentSourceURL":{"index":[1],"value":[6]},"originURL":empty},
   "layout":{"nodeIndex":[],"styles":[],"bounds":[],"text":[],"stackingContexts":flags,"offsetRects":[],"scrollRects":[],"clientRects":[]},
   "textBoxes":{"layoutIndex":[],"bounds":[],"start":[],"length":[]}}]});

    let source = raw.clone();
    let scope = DocumentsScope {
        scope_id: r.context.scope_id.clone(),
        documents: vec![DocumentSeed {
            surface,
            document_backend_id: 7,
            sensitivity: Sensitivity::Public,
        }],
        max_visited_nodes: 16,
    };
    let o = normalize::observation(
        &r.context,
        &r.clock_domain,
        "web.dom",
        (1, 1),
        [1., 2.],
        true,
        false,
    );
    let l = Limits {
        max_nodes: 16,
        max_methods: 100,
        max_reply_bytes: 32768,
        max_total_reply_bytes: 262144,
        max_text_bytes: 16384,
        max_handle_bytes: 256,
        max_ax_properties: 32,
        io_read_bytes: 65536,
        io_write_bytes: 32768,
        io_work: 8192,
    };
    // Exact independent counterexample: both srcset and currentSourceURL.
    assert!(
        snapshot(
            serde_json::from_value(raw).unwrap(),
            &scope,
            &r,
            &[2],
            o.clone(),
            (1, 1),
            l
        )
        .is_err()
    );
    for url_field in ["srcset", "currentSourceURL", "originURL"] {
        for private in [false, true] {
            raw = source.clone();
            raw["documents"][0]["nodes"]["currentSourceURL"] =
                serde_json::json!({"index":[],"value":[]});
            raw["documents"][0]["nodes"]["attributes"][1] = serde_json::json!([]);
            if url_field == "srcset" {
                raw["documents"][0]["nodes"]["attributes"][1] = serde_json::json!([2, 3]);
                raw["strings"][3]=if private {"/public.png 1x, /image?token=Q01_SYNTHETIC_URL_CANARY 2x"} else {"data:image/gif;base64,R0lGODlhAQABAIAAAAAAAP///ywAAAAAAQABAAACAUwAOw== 1x, /public.png?size=2 2x"}.into();
            } else {
                raw["documents"][0]["nodes"][url_field] =
                    serde_json::json!({"index":[1],"value":[6]});
                raw["strings"][6] = if private {
                    "http://127.0.0.1/image?token=Q01_SYNTHETIC_URL_CANARY"
                } else {
                    "http://127.0.0.1/public.png?size=2"
                }
                .into();
            }
            let result = snapshot(
                serde_json::from_value(raw.clone()).unwrap(),
                &scope,
                &r,
                &[2],
                o.clone(),
                (1, 1),
                l,
            );
            if private {
                assert!(
                    matches!(
                        result,
                        Err(Failure {
                            kind: ErrorKind::InvalidInput,
                            ..
                        })
                    ),
                    "private {url_field}"
                );
            } else {
                let result = result.expect("safe URL facts preserved");
                let name = if url_field == "srcset" {
                    "attribute.0.value"
                } else {
                    url_field
                };
                let property = &result.nodes[1]
                    .extensions
                    .iter()
                    .find(|e| e.name.0 == name)
                    .unwrap()
                    .property;
                let index = if url_field == "srcset" { 3 } else { 6 };
                assert_eq!(
                    property.known(),
                    Some(&Value::Text(raw["strings"][index].as_str().unwrap().into()))
                );
            }
        }
    }
    #[derive(serde::Deserialize)]
    struct Case {
        name: String,
        value: String,
        #[serde(rename = "private")]
        blocked: bool,
    }
    let cases: Vec<Case> = serde_json::from_str(include_str!(
        "../../tests/fixtures/collector/srcset-cases.json"
    ))
    .unwrap();
    for case in cases {
        raw = source.clone();
        raw["strings"][3] = case.value.clone().into();
        // The selected resource is safe. Classification must cover ALL candidates.
        raw["strings"][6] = "http://127.0.0.1/public.png".into();
        let result = snapshot(
            serde_json::from_value(raw).unwrap(),
            &scope,
            &r,
            &[2],
            o.clone(),
            (1, 1),
            l,
        );
        if case.blocked {
            assert!(
                matches!(
                    result,
                    Err(Failure {
                        kind: ErrorKind::InvalidInput,
                        ..
                    })
                ),
                "{}",
                case.name
            );
        } else {
            let result = result.expect("safe candidate list");
            let value = &result.nodes[1]
                .extensions
                .iter()
                .find(|e| e.name.0 == "attribute.0.value")
                .unwrap()
                .property;
            assert_eq!(
                value.known(),
                Some(&Value::Text(case.value)),
                "{}",
                case.name
            );
        }
    }
}
