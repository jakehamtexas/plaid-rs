
use serde::{Serialize, Deserialize};
use super::WalletTransaction;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WalletTransactionListResponse {
    pub next_cursor: Option<String>,
    pub request_id: String,
    pub transactions: Vec<WalletTransaction>,
}
impl std::fmt::Display for WalletTransactionListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}