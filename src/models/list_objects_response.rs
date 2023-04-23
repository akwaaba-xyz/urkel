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
pub struct ListObjectsResponse {
    #[serde(rename = "objects", skip_serializing_if = "Option::is_none")]
    pub objects: Option<Vec<String>>,
}

impl ListObjectsResponse {
    pub fn new() -> ListObjectsResponse {
        ListObjectsResponse { objects: None }
    }
}
