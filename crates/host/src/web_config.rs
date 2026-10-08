//! Trusted worker setup and explicit read selection. These are private IPC
//! configuration records, not another Snapshot/Request graph or public CLI wire.
//! Parent transports bytes only; decoding and provider construction are guarded.
use serde::{Deserialize, Serialize};
use uiblueprint_schema::model::{BackendRef, Id, Identity, Sensitivity};
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WebSetup {
    pub endpoint: String,
    pub cdp_session_id: Option<Id>,
    pub surface: Identity,
    pub transport: TransportCaps,
    pub cdp: CdpCaps,
    pub collector: CollectorCaps,
}
#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransportCaps {
    pub endpoint_bytes: usize,
    pub handshake_bytes: usize,
    pub read_buffer_bytes: usize,
    pub write_buffer_bytes: usize,
    pub write_buffer_max: usize,
    pub frame_bytes: usize,
    pub message_bytes: usize,
    pub outbound_bytes: usize,
}
impl TransportCaps {
    pub fn limits(self) -> uiblueprint_web::transport::Limits {
        uiblueprint_web::transport::Limits {
            endpoint_bytes: self.endpoint_bytes,
            handshake_bytes: self.handshake_bytes,
            read_buffer_bytes: self.read_buffer_bytes,
            write_buffer_bytes: self.write_buffer_bytes,
            write_buffer_max: self.write_buffer_max,
            frame_bytes: self.frame_bytes,
            message_bytes: self.message_bytes,
            outbound_bytes: self.outbound_bytes,
        }
    }
}
#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CdpCaps {
    pub max_request_bytes: usize,
    pub max_message_bytes: usize,
    pub max_metadata_bytes: usize,
    pub max_results: usize,
    pub result_bytes: usize,
    pub max_events: usize,
    pub event_bytes: usize,
}
impl CdpCaps {
    pub fn limits(self) -> uiblueprint_web::cdp::Limits {
        uiblueprint_web::cdp::Limits {
            max_request_bytes: self.max_request_bytes,
            max_message_bytes: self.max_message_bytes,
            max_metadata_bytes: self.max_metadata_bytes,
            max_results: self.max_results,
            result_bytes: self.result_bytes,
            max_events: self.max_events,
            event_bytes: self.event_bytes,
        }
    }
}
#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CollectorCaps {
    pub max_nodes: usize,
    pub max_methods: u32,
    pub max_reply_bytes: usize,
    pub max_total_reply_bytes: usize,
    pub max_text_bytes: usize,
    pub max_handle_bytes: usize,
    pub max_ax_properties: usize,
    pub io_read_bytes: usize,
    pub io_write_bytes: usize,
    pub io_work: usize,
}
impl CollectorCaps {
    pub fn limits(self) -> uiblueprint_web::collector::Limits {
        uiblueprint_web::collector::Limits {
            max_nodes: self.max_nodes,
            max_methods: self.max_methods,
            max_reply_bytes: self.max_reply_bytes,
            max_total_reply_bytes: self.max_total_reply_bytes,
            max_text_bytes: self.max_text_bytes,
            max_handle_bytes: self.max_handle_bytes,
            max_ax_properties: self.max_ax_properties,
            io_read_bytes: self.io_read_bytes,
            io_write_bytes: self.io_write_bytes,
            io_work: self.io_work,
        }
    }
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WebId {
    pub id: Id,
    pub sensitivity: Sensitivity,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WebRef {
    pub reference: BackendRef,
    pub sensitivity: Sensitivity,
}
/// Private caller-observed read-only seed, never a fabricated canonical ref.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WebRootSeed {
    pub session_id: Id,
    pub target: Identity,
    pub surface: Identity,
    pub document_backend_id: u32,
    pub backend_node_id: u32,
    pub sensitivity: Sensitivity,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WebDocument {
    pub surface: Identity,
    pub document_backend_id: u32,
    pub sensitivity: Sensitivity,
}
#[derive(Serialize, Deserialize)]
#[serde(tag = "selection", rename_all = "snake_case", deny_unknown_fields)]
pub enum WebSelection {
    Documents {
        documents: Vec<WebDocument>,
        max_visited_nodes: u32,
    },
    Rooted {
        root: WebRootSeed,
        max_visited_nodes: u32,
    },
    Initial {
        ids: Vec<WebId>,
        max_visited_nodes: u32,
    },
    References {
        nodes: Vec<WebRef>,
    },
}

impl WebSetup {
    /// Only trusted attach configuration calls this under the worker guard.
    pub fn decode(bytes: &[u8], limit: usize) -> Result<Self, crate::HostError> {
        decode(bytes, limit)
    }
}
impl WebSelection {
    /// Explicit bounded per-operation selection; no endpoint/authority fields.
    pub fn decode(bytes: &[u8], limit: usize) -> Result<Self, crate::HostError> {
        let selection = decode(bytes, limit)?;
        if let Self::Rooted {
            root,
            max_visited_nodes,
        } = &selection
            && (*max_visited_nodes == 0
                || root.backend_node_id == 0
                || root.backend_node_id > i32::MAX as u32
                || root.document_backend_id == 0
                || root.document_backend_id > i32::MAX as u32
                || [
                    &root.session_id,
                    &root.target.id,
                    &root.target.generation,
                    &root.surface.id,
                    &root.surface.generation,
                ]
                .iter()
                .any(|id| id.0.is_empty() || id.0.chars().count() > 256))
        {
            return Err(crate::HostError::InvalidInput);
        }
        Ok(selection)
    }
}
fn decode<T: serde::de::DeserializeOwned>(
    bytes: &[u8],
    limit: usize,
) -> Result<T, crate::HostError> {
    if bytes.len() > limit {
        return Err(crate::HostError::ResourceLimit);
    }
    if bytes.iter().copied().find(|b| !b" \t\r\n".contains(b)) != Some(b'{') {
        return Err(crate::HostError::InvalidInput);
    }
    serde_json::from_slice(bytes).map_err(|_| crate::HostError::InvalidInput)
}

#[cfg(test)]
mod rooted_tests {
    use super::*;
    #[test]
    fn rooted_seed_is_bounded_private_configuration_not_a_backend_ref() {
        let valid = serde_json::json!({"selection":"rooted","root":{
            "session_id":"s","target":{"id":"t","generation":"g"},"surface":{"id":"f","generation":"d"},
            "document_backend_id":1,"backend_node_id":11,"sensitivity":"public"},"max_visited_nodes":256});
        let bytes = serde_json::to_vec(&valid).unwrap();
        assert!(matches!(
            WebSelection::decode(&bytes, 4096),
            Ok(WebSelection::Rooted { .. })
        ));
        assert!(WebSelection::decode(&bytes, bytes.len() - 1).is_err());
        for mode in 0..5 {
            let mut invalid = valid.clone();
            match mode {
                0 => invalid["root"]["backend_node_id"] = 0.into(),
                1 => invalid["root"]["document_backend_id"] = 0.into(),
                2 => invalid["root"]["snapshot_id"] = "fake".into(),
                3 => invalid["root"]["session_id"] = "".into(),
                _ => invalid["max_visited_nodes"] = 0.into(),
            };
            assert!(WebSelection::decode(&serde_json::to_vec(&invalid).unwrap(), 4096).is_err());
        }
    }
}
