
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Status {
    #[serde(rename = "StatusCode")]
    pub status_code: Option<String>,
    #[serde(rename = "StatusDescription")]
    pub status_description: Option<String>,
}
impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}