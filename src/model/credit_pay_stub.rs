
use serde::{Serialize, Deserialize};
use super::{
    CreditPayStubEmployee, CreditPayStubEarnings, CreditPayStubEmployer,
    CreditDocumentMetadata, CreditPayStubNetPay, PayStubPayPeriodDetails,
    CreditPayStubDeductions,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditPayStub {
    pub deductions: CreditPayStubDeductions,
    pub document_id: Option<String>,
    pub document_metadata: CreditDocumentMetadata,
    pub earnings: CreditPayStubEarnings,
    pub employee: CreditPayStubEmployee,
    pub employer: CreditPayStubEmployer,
    pub net_pay: CreditPayStubNetPay,
    pub pay_period_details: PayStubPayPeriodDetails,
}
impl std::fmt::Display for CreditPayStub {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}