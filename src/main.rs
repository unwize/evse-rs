mod errors;
mod evse;
mod evse_state;
mod wsc;

use ocpp_rs::messages::*;
use crate::wsc::WebsocketClient;

#[tokio::main]
async fn main() {
    let mut wc = WebsocketClient::new("ws://127.0.0.1:8080");
    wc.connect().await.unwrap();

    loop {}
}
