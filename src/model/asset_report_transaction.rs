
use serde::{Serialize, Deserialize};
use super::{TransactionBase, CreditCategory};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetReportTransaction {
    #[serde(flatten)]
    pub transaction_base: TransactionBase,
    pub credit_category: Option<CreditCategory>,
    pub date_transacted: Option<String>,
}
impl std::fmt::Display for AssetReportTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for AssetReportTransaction {
    type Target = TransactionBase;
    fn deref(&self) -> &Self::Target {
        &self.transaction_base
    }
}
impl std::ops::DerefMut for AssetReportTransaction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.transaction_base
    }
}