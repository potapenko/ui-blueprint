//! Explicit small fixture configuration and canonical inputs, not host authority
//! inferred from UI output. All limits are synthetic test parameters.
use uiblueprint_host::{HostLimits, web_config::*};
use uiblueprint_schema::{SchemaVersion, model::*};
pub fn id(s: &str) -> Id {
    Id(s.into())
}
pub fn target() -> Identity {
    Identity {
        id: id("web-target"),
        generation: id("web-generation"),
    }
}
pub fn surface() -> Identity {
    Identity {
        id: id("frame"),
        generation: id("loader"),
    }
}
pub fn descriptor() -> Document {
    Document {
        schema_version: SchemaVersion::CURRENT,
        artifact: Artifact::Session(Box::new(SessionDescriptor {
            session_id: id("web-session"),
            plugin: PluginIdentity {
                id: id("web"),
                version: id("0.1.0"),
            },
            supported_versions: vec![SchemaVersion::CURRENT],
            target: target(),
            surfaces: vec![surface()],
            allowed_scopes: vec![id("scope")],
            capabilities: vec![
                Capability {
                    channel: Channel::ExternalSemantics,
                    operation: id("observe"),
                    status: CapabilityStatus::Partial,
                    reason: Some(id("synthetic-scoped-source")),
                },
                Capability {
                    channel: Channel::RenderedCapture,
                    operation: id("observe"),
                    status: CapabilityStatus::Unsupported,
                    reason: Some(id("web-capture-not-implemented")),
                },
            ],
        })),
    }
}
pub fn request(clock: &str, channels: Vec<Channel>) -> Document {
    Document {
        schema_version: SchemaVersion::CURRENT,
        artifact: Artifact::Request(Box::new(Request {
            clock_domain: id(clock),
            request_id: id("web-request"),
            context: Context {
                schema_version: SchemaVersion::CURRENT,
                session_id: id("web-session"),
                target: target(),
                surfaces: vec![surface()],
                scope_id: id("scope"),
                projection: Projection::Interaction,
                fields: vec![
                    Field::Role,
                    Field::AccessibilityName,
                    Field::LayoutBounds,
                    Field::Value,
                    Field::Checked,
                ],
                plugin: PluginIdentity {
                    id: id("web"),
                    version: id("0.1.0"),
                },
                environment_revision: id("environment"),
            },
            limits: Limits {
                max_elements: 32,
                max_depth: 8,
                max_output_bytes: 65536,
                deadline_ms: 2000,
            },
            freshness_policy: FreshnessPolicy::CurrentRequired,
            operation: Operation::Observe { channels },
        })),
    }
}
pub fn setup(endpoint: String) -> WebSetup {
    WebSetup {
        endpoint,
        cdp_session_id: None,
        surface: surface(),
        transport: TransportCaps {
            endpoint_bytes: 1024,
            handshake_bytes: 2048,
            read_buffer_bytes: 64,
            write_buffer_bytes: 64,
            write_buffer_max: 32768,
            frame_bytes: 8192,
            message_bytes: 8192,
            outbound_bytes: 16384,
        },
        cdp: CdpCaps {
            max_request_bytes: 16384,
            max_message_bytes: 8192,
            max_metadata_bytes: 256,
            max_results: 1,
            result_bytes: 8192,
            max_events: 4,
            event_bytes: 34000,
        },
        collector: CollectorCaps {
            max_nodes: 16,
            max_methods: 100,
            max_reply_bytes: 8192,
            max_total_reply_bytes: 65536,
            max_text_bytes: 600,
            max_handle_bytes: 256,
            max_ax_properties: 32,
            io_read_bytes: 16384,
            io_write_bytes: 32768,
            io_work: 2048,
        },
    }
}
pub fn selection(sensitive: bool) -> WebSelection {
    WebSelection::Initial {
        ids: vec![WebId {
            id: id("left"),
            sensitivity: if sensitive {
                Sensitivity::Sensitive
            } else {
                Sensitivity::Public
            },
        }],
        max_visited_nodes: 32,
    }
}
pub fn limits() -> HostLimits {
    const MIB: usize = 1_048_576;
    HostLimits {
        workers: 2,
        worker_bytes: 64 * MIB,
        publication_reserve: MIB,
        bootstrap_bytes: MIB,
        parent_bytes: 32 * MIB,
        input_bytes: 2 * MIB,
        ingress_bytes: 512 * 1024,
        output_bytes: 512 * 1024,
        request_output_bytes: 2 * MIB,
        completion_groups: 2,
        control_bytes: 4096,
        cleanup_ms: 1000,
        retained_domain_bytes: 64 * MIB,
        retained_per_worker: 15 * MIB,
        main_stack_bytes: 8 * MIB,
        watchdog_stack_bytes: MIB,
    }
}
