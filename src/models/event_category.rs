/*
 * Reclaim SDK API
 *
 * Unofficial Reclaim.ai API based on the provided Python SDK.
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventCategory {
    #[serde(rename = "WORK")]
    Work,
    #[serde(rename = "PERSONAL")]
    Personal,

}

impl std::fmt::Display for EventCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Work => write!(f, "WORK"),
            Self::Personal => write!(f, "PERSONAL"),
        }
    }
}

impl Default for EventCategory {
    fn default() -> EventCategory {
        Self::Work
    }
}

