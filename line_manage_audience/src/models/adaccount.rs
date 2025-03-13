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

/// Adaccount : Adaccount
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Adaccount {
    /// Ad account name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Adaccount {
    /// Adaccount
    pub fn new() -> Adaccount {
        Adaccount {
            name: None,
        }
    }
}

