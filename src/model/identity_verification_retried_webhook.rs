
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityVerificationRetriedWebhook {
    pub environment: String,
    pub identity_verification_id: String,
    pub webhook_code: String,
    pub webhook_type: String,
}
impl std::fmt::Display for IdentityVerificationRetriedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}