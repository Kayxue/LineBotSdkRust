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

use super::{AudienceRecipient, OperatorRecipient, RedeliveryRecipient};

/// Recipient : Recipient
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Recipient {
    #[serde(rename = "audience")]
    AudienceRecipient(AudienceRecipient),
    #[serde(rename = "operator")]
    OperatorRecipient(OperatorRecipient),
    #[serde(rename = "redelivery")]
    RedeliveryRecipient(RedeliveryRecipient),
}

impl Default for Recipient {
    fn default() -> Self {
        Self::AudienceRecipient(AudienceRecipient::default())
    }
}
