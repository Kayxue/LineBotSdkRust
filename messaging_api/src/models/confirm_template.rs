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
pub struct ConfirmTemplate {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "actions")]
    pub actions: Vec<models::Action>,
}

impl ConfirmTemplate {
    pub fn new(r#type: String, text: String, actions: Vec<models::Action>) -> ConfirmTemplate {
        ConfirmTemplate {
            r#type: Some(r#type),
            text,
            actions,
        }
    }
}
