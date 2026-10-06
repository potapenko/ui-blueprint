use std::collections::BTreeMap;
use uiblueprint_schema::model::Snapshot;

/// Kahn's topological pass visits each edge once, retaining the longest path
/// reaching each child. A global visited-only DFS would lose longer shared paths.
/// Roots have depth one. Cycles/dangling refs fail closed even if called before
/// semantic validation. Work is O((V+E) log V), storage O(V), without recursion.
pub(super) fn depth_within(snapshot: &Snapshot, limit: u32) -> bool {
    let nodes = &snapshot.nodes;
    let indices: BTreeMap<_, _> = nodes.iter().enumerate().map(|(i, n)| (&n.key, i)).collect();
    if indices.len() != nodes.len() {
        return false;
    }
    let mut incoming = vec![0_usize; nodes.len()];
    for node in nodes {
        for child in &node.children {
            let Some(&i) = indices.get(child) else {
                return false;
            };
            let Some(count) = incoming[i].checked_add(1) else {
                return false;
            };
            incoming[i] = count;
        }
    }
    let mut ready: Vec<_> = (0..nodes.len()).filter(|&i| incoming[i] == 0).collect();
    let mut depths = vec![1_u32; nodes.len()];
    let mut visited = 0;
    while let Some(i) = ready.pop() {
        visited += 1;
        if depths[i] > limit {
            return false;
        }
        for child in &nodes[i].children {
            // Every key was checked above; no model mutation occurs in this pass.
            let child = indices[child];
            let Some(depth) = depths[i].checked_add(1) else {
                return false;
            };
            depths[child] = depths[child].max(depth);
            incoming[child] -= 1;
            if incoming[child] == 0 {
                ready.push(child);
            }
        }
    }
    visited == nodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use uiblueprint_schema::model::{Artifact, Document, FocusRef, Id};

    fn graph(children: &[Vec<usize>]) -> Snapshot {
        let document: Document = serde_json::from_str(include_str!(
            "../../../fixtures/golden/ENV-SNAPSHOT-VALID.json"
        ))
        .expect("authored canonical fixture");
        let Artifact::Snapshot(mut snapshot) = document.artifact else {
            panic!("snapshot fixture")
        };
        let template = snapshot.nodes[0].clone();
        let keys: Vec<_> = (0..children.len())
            .map(|i| {
                let mut key = template.key.clone();
                key.key = Id(format!("authored-dag-{i}"));
                key
            })
            .collect();
        snapshot.nodes = children
            .iter()
            .enumerate()
            .map(|(i, children)| {
                let mut n = template.clone();
                n.key = keys[i].clone();
                n.children = children.iter().map(|&i| keys[i].clone()).collect();
                n
            })
            .collect();
        snapshot.relations.clear();
        snapshot.components.clear();
        snapshot.surface_records.clear();
        let unknown = FocusRef::Unknown {
            reason: Id("authored_graph".into()),
        };
        snapshot.focus.keyboard = unknown.clone();
        snapshot.focus.accessibility = unknown.clone();
        snapshot.focus.active_descendant = unknown;
        *snapshot
    }

    #[test]
    fn shared_children_use_the_longest_path_independent_of_visit_order() {
        // 0->3 is short; 1->2->3->4 has depth four. Visiting 3 through 0
        // must not suppress the longer path when it is subsequently encountered.
        let s = graph(&[vec![3], vec![2], vec![3], vec![4], vec![]]);
        assert!(depth_within(&s, 4));
        assert!(!depth_within(&s, 3));
        assert!(depth_within(&graph(&[vec![]]), 1));
        assert!(!depth_within(&graph(&[vec![]]), 0));
    }

    #[test]
    fn layered_shared_dag_is_valid_and_does_not_expand_exponential_paths() {
        let edges: Vec<_> = (0..64)
            .map(|i| {
                if i / 2 == 31 {
                    vec![]
                } else {
                    vec![2 * (i / 2 + 1), 2 * (i / 2 + 1) + 1]
                }
            })
            .collect();
        let s = graph(&edges);
        uiblueprint_schema::validation::validate_snapshot(&s)
            .expect("admitted acyclic shared graph");
        assert!(depth_within(&s, 32));
        assert!(!depth_within(&s, 31));
    }

    #[test]
    fn cycles_and_dangling_children_fail_closed() {
        let s = graph(&[vec![1], vec![0]]);
        assert!(!depth_within(&s, u32::MAX));
        assert!(uiblueprint_schema::validation::validate_snapshot(&s).is_err());
        let mut s = graph(&[vec![]]);
        let mut missing = s.nodes[0].key.clone();
        missing.key = Id("missing".into());
        s.nodes[0].children.push(missing);
        assert!(!depth_within(&s, 2));
    }
}
