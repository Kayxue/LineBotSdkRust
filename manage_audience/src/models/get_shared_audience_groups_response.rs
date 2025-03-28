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

/// GetSharedAudienceGroupsResponse : Gets data for more than one audience.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetSharedAudienceGroupsResponse {
    /// An array of audience data. If there are no audiences that match the specified filter, an empty array will be returned.
    #[serde(rename = "audienceGroups", skip_serializing_if = "Option::is_none")]
    pub audience_groups: Option<Vec<models::AudienceGroup>>,
    /// true when this is not the last page.
    #[serde(rename = "hasNextPage", skip_serializing_if = "Option::is_none")]
    pub has_next_page: Option<bool>,
    /// The total number of audiences that can be returned with the specified filter.
    #[serde(rename = "totalCount", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// Of the audiences you can get with the specified filter, the number of audiences with the update permission set to READ_WRITE.
    #[serde(rename = "readWriteAudienceGroupTotalCount", skip_serializing_if = "Option::is_none")]
    pub read_write_audience_group_total_count: Option<i64>,
    /// The current page number.
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// The maximum number of audiences on the current page.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

impl GetSharedAudienceGroupsResponse {
    /// Gets data for more than one audience.
    pub fn new() -> GetSharedAudienceGroupsResponse {
        GetSharedAudienceGroupsResponse {
            audience_groups: None,
            has_next_page: None,
            total_count: None,
            read_write_audience_group_total_count: None,
            page: None,
            size: None,
        }
    }
}

