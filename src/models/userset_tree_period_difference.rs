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
pub struct UsersetTreePeriodDifference {
    #[serde(rename = "base", skip_serializing_if = "Option::is_none")]
    pub base: Option<Box<crate::models::Node>>,
    #[serde(rename = "subtract", skip_serializing_if = "Option::is_none")]
    pub subtract: Option<Box<crate::models::Node>>,
}

impl UsersetTreePeriodDifference {
    pub fn new() -> UsersetTreePeriodDifference {
        UsersetTreePeriodDifference {
            base: None,
            subtract: None,
        }
    }
}


