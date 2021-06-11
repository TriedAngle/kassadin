use crate::WebSocket;
use anyhow::Result;

use native_tls::{TlsConnector, TlsConnectorBuilder};
use websocket::header::{Authorization, Headers};
use websocket::{ClientBuilder, Message, OwnedMessage, WebSocketError};

impl<'a> WebSocket<'a> {
    pub async fn run(self) {
        let address = self.lcu.socket_url();
        let base64_auth = self.lcu.base64_auth();
        let mut headers = Headers::new();
        headers.set(Authorization(format!("Basic {}", base64_auth)));
        let tls_connector = TlsConnector::builder()
            .danger_accept_invalid_hostnames(true)
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();

        let mut client = ClientBuilder::new(&address)
            .unwrap()
            .custom_headers(&headers)
            .connect_secure(Some(tls_connector))
            .unwrap();

        client
            .send_message(&Message::text("[5, \"OnJsonApiEvent\"]"))
            .unwrap();

        for message in client.incoming_messages() {
            let message = message.unwrap();
            println!("message: {:?}", message);
        }
    }
}
