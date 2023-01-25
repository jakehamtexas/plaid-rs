
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemStatusTransactions {
    pub last_failed_update: Option<String>,
    pub last_successful_update: Option<String>,
}
impl std::fmt::Display for ItemStatusTransactions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}