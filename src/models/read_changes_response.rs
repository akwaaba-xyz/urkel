/*
 * OpenFGA
 *
 * A high performance and flexible authorization/permission engine built for developers and inspired by Google Zanzibar.
 *
 * The version of the OpenAPI document: 0.1
 * Contact: community@openfga.dev
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ReadChangesResponse {
    #[serde(rename = "changes", skip_serializing_if = "Option::is_none")]
    pub changes: Option<Vec<crate::models::TupleChange>>,
    #[serde(rename = "continuation_token", skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}

impl ReadChangesResponse {
    pub fn new() -> ReadChangesResponse {
        ReadChangesResponse {
            changes: None,
            continuation_token: None,
        }
    }
}


