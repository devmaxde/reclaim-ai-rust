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
pub enum TaskStatus {
    #[serde(rename = "NEW")]
    New,
    #[serde(rename = "SCHEDULED")]
    Scheduled,
    #[serde(rename = "IN_PROGRESS")]
    InProgress,
    #[serde(rename = "COMPLETE")]
    Complete,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "ARCHIVED")]
    Archived,

}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::New => write!(f, "NEW"),
            Self::Scheduled => write!(f, "SCHEDULED"),
            Self::InProgress => write!(f, "IN_PROGRESS"),
            Self::Complete => write!(f, "COMPLETE"),
            Self::Cancelled => write!(f, "CANCELLED"),
            Self::Archived => write!(f, "ARCHIVED"),
        }
    }
}

impl Default for TaskStatus {
    fn default() -> TaskStatus {
        Self::New
    }
}
