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
pub struct ReadAssertionsResponse {
    #[serde(rename = "authorization_model_id", skip_serializing_if = "Option::is_none")]
    pub authorization_model_id: Option<String>,
    #[serde(rename = "assertions", skip_serializing_if = "Option::is_none")]
    pub assertions: Option<Vec<crate::models::Assertion>>,
}

impl ReadAssertionsResponse {
    pub fn new() -> ReadAssertionsResponse {
        ReadAssertionsResponse {
            authorization_model_id: None,
            assertions: None,
        }
    }
}


