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
pub struct FlexMessage {
    /// Type of message
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "quickReply", skip_serializing_if = "Option::is_none")]
    pub quick_reply: Option<Box<models::QuickReply>>,
    #[serde(rename = "sender", skip_serializing_if = "Option::is_none")]
    pub sender: Option<Box<models::Sender>>,
    #[serde(rename = "altText")]
    pub alt_text: String,
    #[serde(rename = "contents")]
    pub contents: Box<models::FlexContainer>,
}

impl FlexMessage {
    pub fn new(r#type: String, alt_text: String, contents: models::FlexContainer) -> FlexMessage {
        FlexMessage {
            r#type: Some(r#type),
            quick_reply: None,
            sender: None,
            alt_text,
            contents: Box::new(contents),
        }
    }
}
