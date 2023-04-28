#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CheckNOfMRequest {
    #[serde(rename = "checks")]
    pub checks: Vec<crate::models::CheckRequest>,
    #[serde(rename = "n")]
    pub num: usize,
}

impl CheckNOfMRequest {
    pub fn new(checks: Vec<crate::models::CheckRequest>, num: usize) -> CheckNOfMRequest {
        CheckNOfMRequest {
            checks: checks,
            num: num,
        }
    }
}
