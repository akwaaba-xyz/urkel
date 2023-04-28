#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BatchCheckResponse {
    #[serde(rename = "allowed", skip_serializing_if = "Option::is_none")]
    pub allowed: Option<bool>,
    #[serde(rename = "_request", skip_serializing_if = "Option::is_none")]
    pub request: Option<crate::models::CheckRequest>,
    #[serde(rename = "err", skip_serializing_if = "Option::is_none")]
    pub err: Option<String>,
}

impl BatchCheckResponse {
    pub fn new() -> BatchCheckResponse {
        BatchCheckResponse {
            allowed: None,
            request: None,
            err: None,
        }
    }
}
