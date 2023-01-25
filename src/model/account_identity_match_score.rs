
use serde::{Serialize, Deserialize};
use super::{
    AddressMatchScore, EmailAddressMatchScore, NameMatchScore, PhoneNumberMatchScore,
    AccountBase,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountIdentityMatchScore {
    #[serde(flatten)]
    pub account_base: AccountBase,
    pub address: Option<AddressMatchScore>,
    pub email_address: Option<EmailAddressMatchScore>,
    pub legal_name: Option<NameMatchScore>,
    pub phone_number: Option<PhoneNumberMatchScore>,
}
impl std::fmt::Display for AccountIdentityMatchScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for AccountIdentityMatchScore {
    type Target = AccountBase;
    fn deref(&self) -> &Self::Target {
        &self.account_base
    }
}
impl std::ops::DerefMut for AccountIdentityMatchScore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.account_base
    }
}