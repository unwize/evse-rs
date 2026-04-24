use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use rootcause::Result;

pub async fn start_ws_client(
    url: &str,
) -> Result<(mpsc::Sender<Message>, mpsc::Receiver<Message>)> {

    // Connect to the WebSocket server
    let (ws_stream, _) = connect_async(url)
        .await
        .expect("Failed to connect to WebSocket");

    // Split the stream into a sender and a receiver
    let (mut ws_write, mut ws_read) = ws_stream.split();

    // Create bounded channels for buffering incoming and outgoing messages
    let (tx_outgoing, mut rx_outgoing) = mpsc::channel::<Message>(100);
    let (tx_incoming, rx_incoming) = mpsc::channel::<Message>(100);

    // 1. Incoming Message Task (Reads from WS, pushes to Buffer)
    tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_read.next().await {
            // Push to the buffer. If the handler (receiver) drops, this loop breaks.
            if tx_incoming.send(msg).await.is_err() {
                break;
            }
        }
    });

    // 2. Outgoing Message Task (Reads from Queue, writes to WS)
    tokio::spawn(async move {
        while let Some(msg) = rx_outgoing.recv().await {
            // Send to the WS. If the connection fails, this loop breaks.
            if ws_write.send(msg).await.is_err() {
                break;
            }
        }
    });

    Ok((tx_outgoing, rx_incoming))
}

/// A basic client for sending and receiving messages. Owns the connection, input queue, and output queue.
#[derive(Debug)]
pub struct WebsocketClient {
    pub address: String,
    tx: mpsc::Sender<Message>
}

impl WebsocketClient {
    pub async fn try_from_address(address: &str) -> Result<Self> {
        let (tx, mut rx) = start_ws_client(address).await?;

        // The Handler Task (Consumes from the incoming buffer)
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                match msg {
                    Message::Text(text) => println!("Handler received: {}", text),
                    Message::Close(_) => println!("Connection closed"),
                    _ => {} // Handle binary, ping, pong, etc.
                }
            }
        });

        Ok(Self {
            address: address.to_string(),
            tx
        })
    }
    
    pub async fn send(&self, message: Message) -> Result<()> {
        self.tx.send(message).await?;
        Ok(())
    }
}

