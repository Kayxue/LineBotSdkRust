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
pub struct TextMessage {
    /// Type of message
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "quickReply", skip_serializing_if = "Option::is_none")]
    pub quick_reply: Option<Box<models::QuickReply>>,
    #[serde(rename = "sender", skip_serializing_if = "Option::is_none")]
    pub sender: Option<Box<models::Sender>>,
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "emojis", skip_serializing_if = "Option::is_none")]
    pub emojis: Option<Vec<models::Emoji>>,
    /// Quote token of the message you want to quote.
    #[serde(rename = "quoteToken", skip_serializing_if = "Option::is_none")]
    pub quote_token: Option<String>,
}

impl TextMessage {
    pub fn new(r#type: String, text: String) -> TextMessage {
        TextMessage {
            r#type: Some(r#type),
            quick_reply: None,
            sender: None,
            text,
            emojis: None,
            quote_token: None,
        }
    }
}
