/*
 * LINE Messaging API
 *
 * This document describes LINE Messaging API.
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClipboardImagemapAction {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "area")]
    pub area: Box<models::ImagemapArea>,
    /// Text that is copied to the clipboard. Max character limit: 1000
    #[serde(rename = "clipboardText")]
    pub clipboard_text: String,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl ClipboardImagemapAction {
    pub fn new(
        r#type: String,
        area: models::ImagemapArea,
        clipboard_text: String,
    ) -> ClipboardImagemapAction {
        ClipboardImagemapAction {
            r#type: Some(r#type),
            area: Box::new(area),
            clipboard_text,
            label: None,
        }
    }
}
