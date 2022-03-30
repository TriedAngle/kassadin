use crate::{WebSocket, WebSocketBuilder};
use anyhow::Result;

use native_tls::{TlsConnector, TlsConnectorBuilder, TlsStream};
use websocket::header::{Authorization, Headers};
use websocket::sync::Client;
use websocket::websocket_base::stream::sync::TcpStream;
use websocket::{ClientBuilder, Message, OwnedMessage, WebSocketError, WebSocketResult};
use websocket::receiver::Receiver;
use websocket::websocket_base::ws::MessageIterator;

impl<'a> WebSocketBuilder<'a> {
    pub fn build(self) -> WebSocket {
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



        WebSocket {
            client,
            lcu: self.lcu.clone(),
        }
    }
}

impl WebSocket {
    pub fn start(&mut self) -> WebSocketResult<()> {
        self.client
            .send_message(&Message::text("[5, \"OnJsonApiEvent\"]"))
    }

    pub fn stop(&mut self) -> tokio::io::Result<()> {
        self.client.shutdown()
    }
}
