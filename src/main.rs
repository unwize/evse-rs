mod errors;
mod evse;
mod evse_state;
mod wsc;

use crate::evse::{AliveEVSE, BaseEVSE, EVSEProperties};
use rootcause::prelude::ResultExt;
use rootcause::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Report>{
    colog::init();
    let mut evse = BaseEVSE::new(EVSEProperties::default(), "ws://127.0.0.1:8000/evse/sim");
    evse.connect_websocket().await.context("Failed to connect to WebSocketServer")?;
    let mut alive_evse: AliveEVSE = evse.into();
    alive_evse.boot().await?;
    loop {}
}
