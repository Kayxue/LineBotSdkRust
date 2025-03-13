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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetStatisticsPerUnitResponseClick {
    /// The URL's serial number.
    #[serde(rename = "seq")]
    pub seq: i64,
    /// URL.
    #[serde(rename = "url")]
    pub url: String,
    /// Number of times the URL in the bubble was opened.
    #[serde(rename = "click", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub click: Option<Option<i64>>,
    /// Number of users that opened the URL in the bubble.
    #[serde(rename = "uniqueClick", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub unique_click: Option<Option<i64>>,
    /// Number of users who opened this url through any link in the message. If another message bubble contains the same URL and a user opens both links, it's counted only once. 
    #[serde(rename = "uniqueClickOfRequest", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub unique_click_of_request: Option<Option<i64>>,
}

impl GetStatisticsPerUnitResponseClick {
    pub fn new(seq: i64, url: String) -> GetStatisticsPerUnitResponseClick {
        GetStatisticsPerUnitResponseClick {
            seq,
            url,
            click: None,
            unique_click: None,
            unique_click_of_request: None,
        }
    }
}

