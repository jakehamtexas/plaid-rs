
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_selection_enabled: Option<bool>,
}
impl std::fmt::Display for LinkTokenCreateRequestUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}