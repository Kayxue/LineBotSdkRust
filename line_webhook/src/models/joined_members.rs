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
pub struct JoinedMembers {
    /// Users who joined. Array of source user objects.
    #[serde(rename = "members")]
    pub members: Vec<models::UserSource>,
}

impl JoinedMembers {
    pub fn new(members: Vec<models::UserSource>) -> JoinedMembers {
        JoinedMembers {
            members,
        }
    }
}

