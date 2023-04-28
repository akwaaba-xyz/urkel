#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CheckHorizontalRequest {
    #[serde(rename = "read_from")]
    pub read_from: Box<crate::models::ObjectRelation>,
    #[serde(rename = "check_for")]
    pub check_for: Box<crate::models::ObjectRelation>,
    #[serde(
        rename = "authorization_model_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub authorization_model_id: Option<String>,
}

impl CheckHorizontalRequest {
    pub fn new(
        read_from: crate::models::ObjectRelation,
        check_for: crate::models::ObjectRelation,
    ) -> CheckHorizontalRequest {
        CheckHorizontalRequest {
            read_from: Box::new(read_from),
            check_for: Box::new(check_for),
            authorization_model_id: None,
        }
    }
}
