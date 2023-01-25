use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct SandboxTransferRepaymentSimulateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
}
impl<'a> SandboxTransferRepaymentSimulateRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<SandboxTransferRepaymentSimulateResponse> {
        let mut r = self.http_client.client.post("/sandbox/transfer/repayment/simulate");
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for SandboxTransferRepaymentSimulateRequest<'a> {
    type Output = httpclient::InMemoryResult<SandboxTransferRepaymentSimulateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}