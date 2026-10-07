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
#[derive(Serialize, Deserialize)]
#[serde(tag = "selection", rename_all = "snake_case", deny_unknown_fields)]
pub enum WebSelection {
    Initial {
        ids: Vec<WebId>,
        max_visited_nodes: u32,
    },
    References {
        nodes: Vec<WebRef>,
    },
}
