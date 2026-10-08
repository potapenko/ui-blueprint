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
    pub document_url: i32,
    pub title: i32,
    #[serde(rename = "baseURL")]
    pub base_url: i32,
    pub content_language: i32,
    pub encoding_name: i32,
    pub public_id: i32,
    pub system_id: i32,
    pub frame_id: i32,
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
    pub value: Vec<i32>,
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
    pub node_name: Vec<i32>,
    pub node_value: Vec<i32>,
    pub backend_node_id: Vec<u32>,
    pub attributes: Vec<Vec<i32>>,
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
    pub text: Vec<i32>,
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
