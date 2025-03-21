/*
 * LINE Messaging API(Insight)
 *
 * This document describes LINE Messaging API(Insight).
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// GetStatisticsPerUnitResponse : Response object for `get statistics per unit`
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetStatisticsPerUnitResponse {
    #[serde(rename = "overview")]
    pub overview: Box<models::GetStatisticsPerUnitResponseOverview>,
    /// Array of information about individual message bubbles.
    #[serde(rename = "messages")]
    pub messages: Vec<models::GetStatisticsPerUnitResponseMessage>,
    /// Array of information about opened URLs in the message.
    #[serde(rename = "clicks")]
    pub clicks: Vec<models::GetStatisticsPerUnitResponseClick>,
}

impl GetStatisticsPerUnitResponse {
    /// Response object for `get statistics per unit`
    pub fn new(overview: models::GetStatisticsPerUnitResponseOverview, messages: Vec<models::GetStatisticsPerUnitResponseMessage>, clicks: Vec<models::GetStatisticsPerUnitResponseClick>) -> GetStatisticsPerUnitResponse {
        GetStatisticsPerUnitResponse {
            overview: Box::new(overview),
            messages,
            clicks,
        }
    }
}

