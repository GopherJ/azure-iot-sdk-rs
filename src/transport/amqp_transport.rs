use crate::message::Message;
use crate::transport::{MessageHandler, Transport};
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub(crate) struct AmqpTransport {}

#[async_trait]
impl Transport for AmqpTransport {
    async fn new(hub_name: String, device_id: String, sas: String) -> Self {
        AmqpTransport {}
    }

    async fn send_message(&mut self, message: Message) {
        unimplemented!()
    }

    async fn send_property_update(&mut self, request_id: &str, body: &str) {
        unimplemented!()
    }

    async fn set_message_handler(&mut self, device_id: &str, handler: MessageHandler) {
        unimplemented!()
    }
}
