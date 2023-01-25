
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityScreeningHitPhoneNumbers {
    pub phone_number: String,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for EntityScreeningHitPhoneNumbers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}