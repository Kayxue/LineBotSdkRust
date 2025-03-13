/*
 * Webhook Type Definition
 *
 * Webhook event definition of the LINE Messaging API
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Mention {
    /// Array of one or more mention objects. Max: 20 mentions
    #[serde(rename = "mentionees")]
    pub mentionees: Vec<models::Mentionee>,
}

impl Mention {
    pub fn new(mentionees: Vec<models::Mentionee>) -> Mention {
        Mention {
            mentionees,
        }
    }
}

