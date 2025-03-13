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
pub struct RichMenuBatchProgressResponse {
    #[serde(rename = "phase")]
    pub phase: models::RichMenuBatchProgressPhase,
    /// The accepted time in milliseconds of the request of batch control the rich menu.  Format: ISO 8601 (e.g. 2023-06-08T10:15:30.121Z) Timezone: UTC 
    #[serde(rename = "acceptedTime")]
    pub accepted_time: String,
    /// The completed time in milliseconds of rich menu batch control. Returned when the phase property is succeeded or failed.  Format: ISO 8601 (e.g. 2023-06-08T10:15:30.121Z) Timezone: UTC 
    #[serde(rename = "completedTime", skip_serializing_if = "Option::is_none")]
    pub completed_time: Option<String>,
}

impl RichMenuBatchProgressResponse {
    pub fn new(phase: models::RichMenuBatchProgressPhase, accepted_time: String) -> RichMenuBatchProgressResponse {
        RichMenuBatchProgressResponse {
            phase,
            accepted_time,
            completed_time: None,
        }
    }
}

