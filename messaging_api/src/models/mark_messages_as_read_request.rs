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
pub struct MarkMessagesAsReadRequest {
    #[serde(rename = "chat")]
    pub chat: Box<models::ChatReference>,
}

impl MarkMessagesAsReadRequest {
    pub fn new(chat: models::ChatReference) -> MarkMessagesAsReadRequest {
        MarkMessagesAsReadRequest {
            chat: Box::new(chat),
        }
    }
}

