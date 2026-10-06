use crate::{ExportError as E, Result, types::*};
use std::collections::BTreeSet;
use uiblueprint_schema::{model::*, validation};

pub(crate) fn require(ok: bool, error: E) -> Result<()> {
    if ok { Ok(()) } else { Err(error) }
}
pub(crate) fn identifier(text: &str) -> bool {
    !text.is_empty()
        && text.len() <= 128
        && text
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"_-.:".contains(&c))
}
/// Defense in depth for explicitly reviewed public metadata. Not a secret detector.
pub(crate) fn public_text(text: &str) -> bool {
    !text.contains("{{")
        && !text.contains("}}")
        && !text.contains("://")
        && !text.contains('/')
        && !text.contains('\\')
        && !text
            .chars()
            .any(|c| c.is_control() && c != '\n' && c != '\t')
}
pub(crate) fn text(text: &str) -> Result<()> {
    require(
        !text.trim().is_empty() && public_text(text),
        E::PrivateContent,
    )
}
impl DrawingBrief {
    /// Parse within an explicit byte bound. No path/URL in this JSON is dereferenced.
    pub fn from_json(bytes: &[u8], limits: ExportLimits) -> Result<Self> {
        require(bytes.len() <= limits.max_input_bytes, E::InputLimit)?;
        let brief: Self = serde_json::from_slice(bytes).map_err(|_| E::InvalidInput)?;
        validate(&brief, limits)?;
        Ok(brief)
    }
}
pub(crate) fn validate(b: &DrawingBrief, l: ExportLimits) -> Result<()> {
    require(
        l.max_input_bytes > 0
            && l.max_output_bytes > 0
            && l.max_components > 0
            && l.components_per_detail > 0,
        E::InputLimit,
    )?;
    crate::package::bounded_json(b, l.max_input_bytes).map_err(|_| E::InputLimit)?;
    require(
        !b.views.is_empty() && b.views.len() <= l.max_views,
        E::InputLimit,
    )?;
    let m = &b.metadata;
    for t in [
        &m.document_id,
        &m.revision,
        &m.title,
        &m.audience,
        &m.language,
        &m.date,
        &m.owner,
        &m.retention,
        &m.page_format,
        &m.output_size,
    ] {
        text(t)?;
    }
    require(!m.specification_refs.is_empty(), E::InvalidInput)?;
    for t in &m.specification_refs {
        text(t)?;
    }
    if m.approval.status != ApprovalStatus::Draft {
        require(
            m.approval.named_record.is_some(),
            E::UnsupportedVerification,
        )?;
    }
    if let Some(t) = &m.approval.named_record {
        text(t)?;
    }
    let mut ids = BTreeSet::new();
    let mut count = 0usize;
    for v in &b.views {
        require(identifier(&v.id) && ids.insert(&v.id), E::InvalidReference)?;
        for t in [
            &v.title,
            &v.state,
            &v.scope,
            &v.environment,
            &v.safe_source_reference,
        ] {
            text(t)?;
        }
        for t in &v.not_depicted {
            text(t)?;
        }
        match &v.source {
            SourceInput::Observed {
                snapshot,
                public_text_fields,
            } => {
                validation::validate_snapshot(snapshot).map_err(|_| E::InvalidSource)?;
                require(
                    public_text_fields.iter().all(|f| {
                        matches!(
                            f,
                            Field::Name
                                | Field::VisibleText
                                | Field::AccessibilityName
                                | Field::Description
                                | Field::Placeholder
                                | Field::Actions
                                | Field::InputKind
                        )
                    }),
                    E::PrivateContent,
                )?;
                count = count
                    .checked_add(snapshot.nodes.len())
                    .ok_or(E::InputLimit)?;
            }
            SourceInput::Proposed { layout } => {
                crate::proposal::validate(layout)?;
                count = count
                    .checked_add(layout.components.len())
                    .ok_or(E::InputLimit)?;
            }
        }
    }
    require(count <= l.max_components, E::InputLimit)?;
    match b.purpose {
        Purpose::Document => require(
            b.views
                .iter()
                .all(|v| matches!(v.source, SourceInput::Observed { .. })),
            E::InvalidSource,
        )?,
        Purpose::Propose => require(
            b.views
                .iter()
                .all(|v| matches!(v.source, SourceInput::Proposed { .. })),
            E::InvalidSource,
        )?,
        Purpose::Detail => require(!b.details.is_empty(), E::InvalidReference)?,
        Purpose::Compare => require(!b.comparisons.is_empty(), E::InvalidReference)?,
        Purpose::Flow => require(!b.transitions.is_empty(), E::InvalidReference)?,
    }
    require(
        b.purpose == Purpose::Flow || b.transitions.is_empty(),
        E::InvalidInput,
    )?;
    for c in &b.comparisons {
        let a = view(b, &c.before)?;
        let z = view(b, &c.after)?;
        require(a.id != z.id, E::InvalidReference)?;
        let compatible = match (&a.source, &z.source) {
            (
                SourceInput::Observed { snapshot: a, .. },
                SourceInput::Observed { snapshot: z, .. },
            ) => validation::contexts_compatible(&a.context, &z.context),
            (SourceInput::Proposed { .. }, SourceInput::Proposed { .. }) => true,
            _ => false,
        };
        require(
            compatible || c.different_basis.is_some(),
            E::IncompatibleViews,
        )?;
        if let Some(t) = &c.different_basis {
            text(t)?;
        }
    }
    Ok(())
}
pub(crate) fn view<'a>(b: &'a DrawingBrief, id: &str) -> Result<&'a ViewInput> {
    b.views
        .iter()
        .find(|v| v.id == id)
        .ok_or(E::InvalidReference)
}
