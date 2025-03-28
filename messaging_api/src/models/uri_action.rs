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
pub struct UriAction {
    /// Type of action
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Label for the action.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(rename = "altUri", skip_serializing_if = "Option::is_none")]
    pub alt_uri: Option<Box<models::AltUri>>,
}

impl UriAction {
    pub fn new() -> UriAction {
        UriAction {
            r#type: None,
            label: None,
            uri: None,
            alt_uri: None,
        }
    }
}

