use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct WalletTransactionGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub transaction_id: String,
}
impl<'a> WalletTransactionGetRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<WalletTransactionGetResponse> {
        let mut r = self.http_client.client.post("/wallet/transaction/get");
        r = r.json(json!({ "transaction_id" : self.transaction_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for WalletTransactionGetRequest<'a> {
    type Output = httpclient::InMemoryResult<WalletTransactionGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}