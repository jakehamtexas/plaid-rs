
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationConsentCreateRequest {
    pub constraints: PaymentInitiationConsentConstraints,
    pub options: Option<ExternalPaymentInitiationConsentOptions>,
    pub recipient_id: String,
    pub reference: String,
    pub scopes: Vec<String>,
}
impl std::fmt::Display for PaymentInitiationConsentCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}