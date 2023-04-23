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
pub struct Difference {
    #[serde(rename = "base")]
    pub base: Box<crate::models::Userset>,
    #[serde(rename = "subtract")]
    pub subtract: Box<crate::models::Userset>,
}

impl Difference {
    pub fn new(base: crate::models::Userset, subtract: crate::models::Userset) -> Difference {
        Difference {
            base: Box::new(base),
            subtract: Box::new(subtract),
        }
    }
}
