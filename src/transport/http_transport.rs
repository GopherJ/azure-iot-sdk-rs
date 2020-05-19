use crate::message::Message;
use crate::transport::{MessageHandler, Transport};
use async_trait::async_trait;
use hyper::{Body, Client, Method, Request, Uri};

#[derive(Debug, Clone)]
pub(crate) struct HttpTransport {
    hub_name: String,
    device_id: String,
    sas: String,
}

impl HttpTransport {
    fn new(hub_name: String, device_id: String, sas: String) -> Self {
        HttpTransport {
            hub_name,
            device_id,
            sas,
        }
    }
}

#[async_trait]
impl Transport for HttpTransport {
    async fn send_message(&mut self, message: Message) {
        let client = Client::new();

        let req = Request::builder()
            .method(Method::POST)
            .uri(format!(
                "https://{}.azure-devices.net/devices/{}/messages/events?api-version=2019-03-30",
                self.hub_name, self.device_id
            ))
            .header("Content-Type", "application/json")
            .body(Body::from(message.body))
            .unwrap();

        let resp = client.request(req).await.unwrap();
    }

    async fn send_property_update(&mut self, request_id: &str, body: &str) {
        unimplemented!()
    }

    async fn set_message_handler(&mut self, device_id: &str, handler: MessageHandler) {
        unimplemented!()
    }
}
