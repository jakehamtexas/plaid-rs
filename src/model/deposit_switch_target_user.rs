
use serde::{Serialize, Deserialize};
use super::DepositSwitchAddressData;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DepositSwitchTargetUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<DepositSwitchAddressData>,
    pub email: String,
    pub family_name: String,
    pub given_name: String,
    pub phone: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_payer_id: Option<String>,
}
impl std::fmt::Display for DepositSwitchTargetUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}