use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PaymentInitiationPaymentGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub payment_id: String,
}
impl<'a> PaymentInitiationPaymentGetRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<PaymentInitiationPaymentGetResponse> {
        let mut r = self.http_client.client.post("/payment_initiation/payment/get");
        r = r.json(json!({ "payment_id" : self.payment_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for PaymentInitiationPaymentGetRequest<'a> {
    type Output = httpclient::InMemoryResult<PaymentInitiationPaymentGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}