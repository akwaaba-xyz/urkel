/*
 * OpenFGA
 *
 * A high performance and flexible authorization/permission engine built for developers and inspired by Google Zanzibar.
 *
 * The version of the OpenAPI document: 0.1
 * Contact: community@openfga.dev
 * Generated by: https://openapi-generator.tech
 */

/// Leaf : A leaf node contains either - a set of users (which may be individual users, or usersets   referencing other relations) - a computed node, which is the result of a computed userset   value in the authorization model - a tupleToUserset nodes, containing the result of expanding   a tupleToUserset value in a authorization model.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Leaf {
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Box<crate::models::Users>>,
    #[serde(rename = "computed", skip_serializing_if = "Option::is_none")]
    pub computed: Option<Box<crate::models::Computed>>,
    #[serde(rename = "tupleToUserset", skip_serializing_if = "Option::is_none")]
    pub tuple_to_userset: Option<Box<crate::models::UsersetTreePeriodTupleToUserset>>,
}

impl Leaf {
    /// A leaf node contains either - a set of users (which may be individual users, or usersets   referencing other relations) - a computed node, which is the result of a computed userset   value in the authorization model - a tupleToUserset nodes, containing the result of expanding   a tupleToUserset value in a authorization model.
    pub fn new() -> Leaf {
        Leaf {
            users: None,
            computed: None,
            tuple_to_userset: None,
        }
    }
}


