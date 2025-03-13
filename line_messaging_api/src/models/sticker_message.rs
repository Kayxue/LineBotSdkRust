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
pub struct StickerMessage {
    /// Type of message
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "quickReply", skip_serializing_if = "Option::is_none")]
    pub quick_reply: Option<Box<models::QuickReply>>,
    #[serde(rename = "sender", skip_serializing_if = "Option::is_none")]
    pub sender: Option<Box<models::Sender>>,
    #[serde(rename = "packageId")]
    pub package_id: String,
    #[serde(rename = "stickerId")]
    pub sticker_id: String,
    /// Quote token of the message you want to quote.
    #[serde(rename = "quoteToken", skip_serializing_if = "Option::is_none")]
    pub quote_token: Option<String>,
}

impl StickerMessage {
    pub fn new(r#type: String, package_id: String, sticker_id: String) -> StickerMessage {
        StickerMessage {
            r#type: Some(r#type),
            quick_reply: None,
            sender: None,
            package_id,
            sticker_id,
            quote_token: None,
        }
    }
}
