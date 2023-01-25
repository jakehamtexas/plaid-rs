
use serde::{Serialize, Deserialize};
use super::TransferTestClock;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxTransferTestClockGetResponse {
    pub request_id: String,
    pub test_clock: TransferTestClock,
}
impl std::fmt::Display for SandboxTransferTestClockGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}