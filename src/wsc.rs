use std::collections::VecDeque;
use std::sync::Arc;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use miette::{IntoDiagnostic, Result};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::RwLock;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;

/// A basic client for sending and receiving messages. Owns the connection, input queue, and output queue.
pub struct WebsocketClient {
    pub address: String,
    tx_stream: Option<RwLock<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    rx_stream: Option<RwLock<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    rx_queue: Arc<RwLock<VecDeque<Message>>>,
    tx_queue: Arc<RwLock<VecDeque<Message>>>,

}

impl WebsocketClient {
    pub fn new(address: &str) -> Self {
        Self { address: String::from(address), tx_stream: None, rx_stream: None, rx_queue: Default::default(), tx_queue: Default::default() }
    }

    pub async fn connect(&mut self) -> Result<()> {
        let mut request = self.address.as_str().into_client_request().into_diagnostic()?;
        request.headers_mut().insert("ocpp", "2.1".parse().unwrap());
        let (ws_stream, _) = connect_async(request).await.expect("Failed to connect");
        println!("WebSocket handshake has been successfully completed");

        let (write, read) = ws_stream.split();
        self.tx_stream = Some(RwLock::new(write));
        self.rx_stream = Some(RwLock::new(read));
        Ok(())
    }

    pub async fn push_message(&mut self, message: Message) -> Result<()> {
        self.tx_queue.write().await.push_back(message);
        Ok(())
    }

    pub async fn send(&mut self, message: Message) -> Result<()> {
        if let Some(tx_lock) = &self.tx_stream {
            let mut tx = tx_lock.write().await;
            (*tx).send(message).await.into_diagnostic()?;
        }

        Ok(())
    }
}

