/*
 * Channel Access Token API
 *
 * This document describes Channel Access Token API.
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// IssueChannelAccessTokenResponse : Issued channel access token
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueChannelAccessTokenResponse {
    /// Channel access token. 
    #[serde(rename = "access_token")]
    pub access_token: String,
    /// Amount of time in seconds from issue to expiration of the channel access token
    #[serde(rename = "expires_in")]
    pub expires_in: i32,
    /// A token type.
    #[serde(rename = "token_type")]
    pub token_type: String,
    /// Unique key ID for identifying the channel access token.
    #[serde(rename = "key_id")]
    pub key_id: String,
}

impl IssueChannelAccessTokenResponse {
    /// Issued channel access token
    pub fn new(access_token: String, expires_in: i32, token_type: String, key_id: String) -> IssueChannelAccessTokenResponse {
        IssueChannelAccessTokenResponse {
            access_token,
            expires_in,
            token_type,
            key_id,
        }
    }
}

