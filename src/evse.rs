use crate::wsc::WebsocketClient;
use ocpp_rs::messages::boot_notification::BootNotificationRequest;
use ocpp_rs::ocppj::RcpCall;
use ocpp_rs::structures::charging_station_type::ChargingStationType;
use ocpp_rs::structures::modem_type::ModemType;
use rootcause::prelude::*;
use serde::Serialize;
use tokio_tungstenite::tungstenite::{Message, Utf8Bytes};
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct EVSEProperties {
    pub serial_number: String,
    pub make: String,
    pub model: String,
    pub firmware_version: String,
    pub modem: Option<ModemType>,
}

impl EVSEProperties {
    pub fn new(
        serial_number: String,
        make: String,
        model: String,
        firmware_version: String,
        modem: Option<ModemType>,
        csms_endpoint: &str
    ) -> Self {
        Self {
            serial_number,
            make,
            model,
            firmware_version,
            modem,
        }
    }
}

#[derive(Debug)]
pub struct BaseEVSE {
    pub properties: EVSEProperties,
    pub websocket: WebsocketClient
}

impl Default for BaseEVSE {
    fn default() -> Self {
        Self {
            properties: EVSEProperties::default(),
            websocket: WebsocketClient::default()
        }
    }
}

impl BaseEVSE {

    pub fn new(properties: EVSEProperties, csms_endpoint: &str) -> Self {
        Self {
            properties,
            websocket: WebsocketClient::new(csms_endpoint),
        }
    }

    pub async fn connect_websocket(&mut self) -> Result<(), Report> {
        self.websocket.connect().await?;
        Ok(())
    }
}

impl Into<AliveEVSE> for BaseEVSE {
    fn into(self) -> AliveEVSE {
        AliveEVSE {
            properties: self.properties,
            websocket: self.websocket,
        }
    }
}

pub struct AliveEVSE {
    pub properties: EVSEProperties,
    pub websocket: WebsocketClient
}

impl AliveEVSE {

    /// Send an OCPP message to the CSMS
    pub async fn send_message(&mut self, message: impl Serialize + std::fmt::Debug) -> Result<(), Report> {
        log::info!("Sending message: {:?}", serde_json::to_string(&message)?);
        self.websocket.send(Message::Text(Utf8Bytes::from(serde_json::to_string(&message).context("Failed to serialize message to JSON")?))).await.context("Failed to send message")?;
        Ok(())
    }

    /// Generate a boot notification
    fn get_boot_notification(&self) -> Result<BootNotificationRequest, Report> {
        Ok(BootNotificationRequest {
            reason: Default::default(),
            charging_station: ChargingStationType {
                serial_number: Some(self.properties.serial_number.clone()),
                model: self.properties.model.clone(),
                vendor_name: self.properties.make.to_string(),
                firmware_version: Some(self.properties.firmware_version.clone()),
                modem: self.properties.modem.clone(),
            },
        })
    }

    /// 1. The Charging Station is powered up.
    /// 2. The Charging Station sends BootNotificationRequest to the CSMS.
    /// 3. The CSMS returns with BootNotificationResponse with the status Accepted.
    /// 4. Optional: The Charging Station sends NotifyEventRequest with component.name Connector,
    ///    variable.name AvailabilityState and actualValue Unavailable to the CSMS for each Connector.
    /// 5. The Charging Station sends NotifyEventRequest with component.name Connector,
    ///    variable.name AvailabilityState to the CSMS for each Connector. If the AvailabilityState was set to
    ///    Unavailable or Reserved from the CSMS prior to the (re)boot, the Connector should return to this
    ///    AvailabilityState, otherwise the AvailabilityState should be Available or, when it resumes a
    ///    transaction that was ongoing, the AvailabilityState should be Occupied.
    /// 6. Normal operation is resumed.
    /// 7. The Charging Station sends HeartbeatRequest to the CSMS.
    pub async fn boot(&mut self) -> Result<(), Report> {
        log::info!("Starting boot process");
        // Generate BootNotificationRequest
        let bnf = self.get_boot_notification()?;
        let message = RcpCall::new(Uuid::new_v4().simple().to_string().as_str(), Box::new(bnf));
        // Dispatch BNF
        self.send_message(message.clone()).await?;

        // Await BootNotificationResponse from CSMS


        Ok(())
    }
}
