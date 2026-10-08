//! Private CDP tables; never serialized or stored as a second graph.
use serde::Deserialize;
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct Capture {
    pub documents: Vec<Document>,
    pub strings: Vec<String>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct Document {
    #[serde(rename = "documentURL")]
    pub document_url: usize,
    pub title: usize,
    #[serde(rename = "baseURL")]
    pub base_url: usize,
    pub content_language: usize,
    pub encoding_name: usize,
    pub public_id: usize,
    pub system_id: usize,
    pub frame_id: usize,
    pub nodes: Nodes,
    pub layout: Layout,
    pub text_boxes: TextBoxes,
    pub scroll_offset_x: Option<f64>,
    pub scroll_offset_y: Option<f64>,
    pub content_width: Option<f64>,
    pub content_height: Option<f64>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Rare {
    pub index: Vec<usize>,
    pub value: Vec<usize>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Flags {
    pub index: Vec<usize>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct Nodes {
    pub parent_index: Vec<i32>,
    pub node_type: Vec<u32>,
    pub node_name: Vec<usize>,
    pub node_value: Vec<usize>,
    pub backend_node_id: Vec<u32>,
    pub attributes: Vec<Vec<usize>>,
    pub shadow_root_type: Rare,
    pub text_value: Rare,
    pub input_value: Rare,
    pub input_checked: Flags,
    pub option_selected: Flags,
    pub content_document_index: Rare,
    pub pseudo_type: Rare,
    pub pseudo_identifier: Rare,
    pub is_clickable: Flags,
    #[serde(rename = "currentSourceURL")]
    pub current_source_url: Rare,
    #[serde(rename = "originURL")]
    pub origin_url: Rare,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct Layout {
    pub node_index: Vec<usize>,
    pub styles: Vec<Vec<usize>>,
    pub bounds: Vec<Vec<f64>>,
    pub text: Vec<usize>,
    pub stacking_contexts: Flags,
    pub offset_rects: Vec<Vec<f64>>,
    pub scroll_rects: Vec<Vec<f64>>,
    pub client_rects: Vec<Vec<f64>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct TextBoxes {
    pub layout_index: Vec<usize>,
    pub bounds: Vec<Vec<f64>>,
    pub start: Vec<u32>,
    pub length: Vec<u32>,
}
