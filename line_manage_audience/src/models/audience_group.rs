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

/// AudienceGroup : Audience group
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AudienceGroup {
    /// The audience ID.
    #[serde(rename = "audienceGroupId", skip_serializing_if = "Option::is_none")]
    pub audience_group_id: Option<i64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::AudienceGroupType>,
    /// The audience's name.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::AudienceGroupStatus>,
    #[serde(rename = "failedType", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub failed_type: Option<Option<models::AudienceGroupFailedType>>,
    /// The number of users included in the audience.
    #[serde(rename = "audienceCount", skip_serializing_if = "Option::is_none")]
    pub audience_count: Option<i64>,
    /// When the audience was created (in UNIX time).
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    /// The request ID that was specified when the audience was created. This is only included when `audienceGroup.type` is CLICK or IMP. 
    #[serde(rename = "requestId", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// The URL that was specified when the audience was created. This is only included when `audienceGroup.type` is CLICK and link URL is specified. 
    #[serde(rename = "clickUrl", skip_serializing_if = "Option::is_none")]
    pub click_url: Option<String>,
    /// The value indicating the type of account to be sent, as specified when creating the audience for uploading user IDs. 
    #[serde(rename = "isIfaAudience", skip_serializing_if = "Option::is_none")]
    pub is_ifa_audience: Option<bool>,
    #[serde(rename = "permission", skip_serializing_if = "Option::is_none")]
    pub permission: Option<models::AudienceGroupPermission>,
    #[serde(rename = "createRoute", skip_serializing_if = "Option::is_none")]
    pub create_route: Option<models::AudienceGroupCreateRoute>,
}

impl AudienceGroup {
    /// Audience group
    pub fn new() -> AudienceGroup {
        AudienceGroup {
            audience_group_id: None,
            r#type: None,
            description: None,
            status: None,
            failed_type: None,
            audience_count: None,
            created: None,
            request_id: None,
            click_url: None,
            is_ifa_audience: None,
            permission: None,
            create_route: None,
        }
    }
}

