use uiblueprint_schema::{
    model::*,
    validation::{ValidationError, validate_delta, validate_snapshot},
};

fn graph(edges: &[Vec<usize>]) -> Snapshot {
    let doc: Document =
        serde_json::from_slice(include_bytes!("../../../fixtures/golden/GEO-GAP.json")).unwrap();
    let Artifact::Finding(case) = doc.artifact else {
        panic!("fixture")
    };
    let mut snapshot = case.snapshot;
    let template = snapshot.nodes[0].clone();
    let keys: Vec<_> = (0..edges.len())
        .map(|index| SourceKey {
            namespace: template.key.namespace.clone(),
            key: Id(format!("node-{index:05}")),
        })
        .collect();
    snapshot.nodes = edges
        .iter()
        .enumerate()
        .map(|(index, children)| {
            let mut node = template.clone();
            node.key = keys[index].clone();
            node.children = children.iter().map(|i| keys[*i].clone()).collect();
            node
        })
        .collect();
    snapshot.relations.clear();
    snapshot.components.clear();
    snapshot.surface_records.clear();
    snapshot.captures.clear();
    snapshot
}

#[test]
fn all_three_node_graphs_match_an_independent_topological_oracle() {
    // Kahn's indegree elimination is independent of the production DFS.
    for bits in 0..512 {
        let edges: Vec<Vec<usize>> = (0..3)
            .map(|from| {
                (0..3)
                    .filter(|to| bits & (1 << (from * 3 + to)) != 0)
                    .collect()
            })
            .collect();
        let mut indegree = [0; 3];
        for children in &edges {
            for &to in children {
                indegree[to] += 1;
            }
        }
        let mut pending: Vec<_> = (0..3).filter(|&n| indegree[n] == 0).collect();
        let mut removed = 0;
        while let Some(n) = pending.pop() {
            removed += 1;
            for &child in &edges[n] {
                indegree[child] -= 1;
                if indegree[child] == 0 {
                    pending.push(child);
                }
            }
        }
        let actual = validate_snapshot(&graph(&edges));
        assert_eq!(
            actual,
            if removed == 3 {
                Ok(())
            } else {
                Err(ValidationError::DanglingReference)
            },
            "graph {bits}"
        );
    }
}

#[test]
fn shared_subgraphs_long_chains_and_source_order_remain_valid() {
    let mut diamond = graph(&[vec![1, 2], vec![3], vec![3], vec![]]);
    validate_snapshot(&diamond).unwrap();
    diamond.nodes.reverse();
    validate_snapshot(&diamond).unwrap();
    let mut edges: Vec<Vec<usize>> = (0..10_000)
        .map(|i| if i == 9_999 { vec![] } else { vec![i + 1] })
        .collect();
    validate_snapshot(&graph(&edges)).unwrap();
    // The disconnected component remains checked even after another root finished.
    edges[5000].clear();
    edges[9999].push(5001);
    assert_eq!(
        validate_snapshot(&graph(&edges)),
        Err(ValidationError::DanglingReference)
    );
}

#[test]
fn duplicate_and_dangling_references_keep_their_error_categories() {
    let mut snapshot = graph(&[vec![1], vec![]]);
    let duplicate = snapshot.nodes[0].children[0].clone();
    snapshot.nodes[0].children.push(duplicate);
    assert_eq!(
        validate_snapshot(&snapshot),
        Err(ValidationError::DuplicateIdentity)
    );
    snapshot.nodes[0].children.pop();
    snapshot.nodes[0].children[0].key = Id("missing".into());
    assert_eq!(
        validate_snapshot(&snapshot),
        Err(ValidationError::DanglingReference)
    );
    snapshot.nodes[0].children.clear();
    snapshot.nodes.push(snapshot.nodes[0].clone());
    assert_eq!(
        validate_snapshot(&snapshot),
        Err(ValidationError::DuplicateIdentity)
    );
}

fn update(base: &Snapshot) -> Delta {
    Delta {
        base_revision: base.revision,
        revision: base.revision + 1,
        source_state: None,
        context: base.context.clone(),
        observations: base.observations.clone(),
        upsert: base.nodes.clone(),
        removed: vec![],
        relations: vec![],
        focus: base.focus.clone(),
        coverage: base.coverage.clone(),
    }
}

#[test]
fn delta_upserts_and_removals_check_the_final_graph_without_publishing_it() {
    let base = graph(&[vec![1], vec![]]);
    let mut delta = update(&base);
    validate_delta(&base, &delta).unwrap();
    delta.upsert[1].children.push(base.nodes[0].key.clone());
    assert_eq!(
        validate_delta(&base, &delta),
        Err(ValidationError::DanglingReference)
    );
    delta.upsert[1].children[0].key = Id("missing".into());
    assert_eq!(
        validate_delta(&base, &delta),
        Err(ValidationError::DanglingReference)
    );
    delta = update(&base);
    let Property::Requested { evidence, .. } = &base.nodes[1].properties[0] else {
        panic!("fixture")
    };
    delta.removed.push(Removal {
        key: base.nodes[1].key.clone(),
        evidence: evidence.clone(),
    });
    delta.upsert.clear();
    assert_eq!(
        validate_delta(&base, &delta),
        Err(ValidationError::DanglingReference)
    );
    let mut parent = base.nodes[0].clone();
    parent.children.clear();
    delta.upsert.push(parent);
    validate_delta(&base, &delta).unwrap();
    delta.upsert.push(base.nodes[1].clone());
    assert_eq!(
        validate_delta(&base, &delta),
        Err(ValidationError::DanglingReference)
    );
}
