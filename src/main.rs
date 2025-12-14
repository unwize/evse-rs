mod errors;
mod evse;
mod evse_state;
mod wsc;

use ocpp_rs::messages::*;
use rootcause::prelude::ResultExt;
use crate::wsc::WebsocketClient;
use rootcause::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Report>{
    let mut wc = WebsocketClient::new("ws://127.0.0.1:8080");
    wc.connect().await.context("Failed to connect to WebSocketServer")?;
    loop {}
}
