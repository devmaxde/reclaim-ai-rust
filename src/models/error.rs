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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    /// Error message
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl Error {
    pub fn new() -> Error {
        Error {
            message: None,
        }
    }
}
