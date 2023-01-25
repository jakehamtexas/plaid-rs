
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditPayrollIncomeRiskSignalsGetRequest {
    pub user_token: Option<String>,
}
impl std::fmt::Display for CreditPayrollIncomeRiskSignalsGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}