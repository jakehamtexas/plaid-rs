
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferUserInRequestDeprecated {
    pub address: Option<TransferUserAddressInRequest>,
    pub email_address: Option<String>,
    pub legal_name: Option<String>,
    pub phone_number: Option<String>,
}
impl std::fmt::Display for TransferUserInRequestDeprecated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}