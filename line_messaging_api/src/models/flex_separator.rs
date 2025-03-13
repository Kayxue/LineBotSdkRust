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
pub struct FlexSeparator {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "margin", skip_serializing_if = "Option::is_none")]
    pub margin: Option<String>,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

impl FlexSeparator {
    pub fn new(r#type: String) -> FlexSeparator {
        FlexSeparator {
            r#type: Some(r#type),
            margin: None,
            color: None,
        }
    }
}
