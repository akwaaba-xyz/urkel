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
pub struct UsersetTreePeriodTupleToUserset {
    #[serde(rename = "tupleset", skip_serializing_if = "Option::is_none")]
    pub tupleset: Option<String>,
    #[serde(rename = "computed", skip_serializing_if = "Option::is_none")]
    pub computed: Option<Vec<crate::models::Computed>>,
}

impl UsersetTreePeriodTupleToUserset {
    pub fn new() -> UsersetTreePeriodTupleToUserset {
        UsersetTreePeriodTupleToUserset {
            tupleset: None,
            computed: None,
        }
    }
}


