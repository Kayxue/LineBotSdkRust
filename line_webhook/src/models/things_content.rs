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

use super::{LinkThingsContent, ScenarioResultThingsContent, UnlinkThingsContent};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ThingsContent {
    #[serde(rename = "link")]
    LinkThingsContent(LinkThingsContent),
    #[serde(rename = "scenarioResult")]
    ScenarioResultThingsContent(ScenarioResultThingsContent),
    #[serde(rename = "unlink")]
    UnlinkThingsContent(UnlinkThingsContent),
}

impl Default for ThingsContent {
    fn default() -> Self {
        Self::LinkThingsContent(LinkThingsContent::default())
    }
}
