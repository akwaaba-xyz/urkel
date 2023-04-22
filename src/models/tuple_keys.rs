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
pub struct TupleKeys {
    #[serde(rename = "tuple_keys")]
    pub tuple_keys: Vec<crate::models::TupleKey>,
}

impl TupleKeys {
    pub fn new(tuple_keys: Vec<crate::models::TupleKey>) -> TupleKeys {
        TupleKeys {
            tuple_keys,
        }
    }
}


