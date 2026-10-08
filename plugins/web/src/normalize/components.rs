use super::*;

/// Only complete, unambiguous declarations within the selected public records
/// qualify. AX membership follows the separately sourced backendDOMNodeId links.
/// Neither mapping nor a decorative member grants an action capability.
pub(crate) fn components(records: &[(u32, DomRead)], snapshot: &mut Snapshot) {
    for (backend, read) in records {
        let Some(declaration) = &read.component else {
            continue;
        };
        if read.sensitive
            || records
                .iter()
                .filter(|(_, r)| {
                    r.component
                        .as_ref()
                        .is_some_and(|c| c.key == declaration.key)
                })
                .count()
                != 1
            || declaration
                .members
                .iter()
                .any(|&i| records.get(i).is_none_or(|(_, r)| r.sensitive))
        {
            continue;
        }
        let owner = dom_key(*backend);
        let mut members = vec![owner.clone()];
        for &index in &declaration.members {
            let key = dom_key(records[index].0);
            if !members.contains(&key) {
                members.push(key);
            }
        }
        let dom_count = members.len();
        for i in 0..dom_count {
            for relation in &snapshot.relations {
                if relation.kind == RelationKind::CorrespondsTo
                    && relation.from == members[i]
                    && relation.to.namespace.0 == "web.ax"
                    && !members.contains(&relation.to)
                {
                    members.push(relation.to.clone());
                }
            }
        }
        let source = Id(format!("dom-data-component-key-parts:{backend}"));
        if let Some(node) = snapshot.nodes.iter_mut().find(|n| n.key == owner) {
            node.source_declarations.push(SourceDeclaration {
                namespace: id("web.dom"),
                name: id("component_key"),
                state: known(Value::Text(declaration.key.clone())),
                sensitivity: Sensitivity::Public,
                source: source.clone(),
            });
        }
        snapshot.components.push(ComponentMapping {
            logical_component_key: Id(declaration.key.clone()),
            members,
            declaration_source: source,
            provenance: Provenance::Reported,
        });
    }
}
