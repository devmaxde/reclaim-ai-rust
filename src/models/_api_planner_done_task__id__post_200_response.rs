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
pub struct ApiPlannerDoneTaskIdPost200Response {
    #[serde(rename = "taskOrHabit", skip_serializing_if = "Option::is_none")]
    pub task_or_habit: Option<Box<models::Task>>,
}

impl ApiPlannerDoneTaskIdPost200Response {
    pub fn new() -> ApiPlannerDoneTaskIdPost200Response {
        ApiPlannerDoneTaskIdPost200Response {
            task_or_habit: None,
        }
    }
}
