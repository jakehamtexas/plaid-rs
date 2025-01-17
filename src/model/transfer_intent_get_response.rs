
use serde::{Serialize, Deserialize};
use super::TransferIntentGet;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferIntentGetResponse {
    pub request_id: String,
    pub transfer_intent: TransferIntentGet,
}
impl std::fmt::Display for TransferIntentGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}