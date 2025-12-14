use ocpp_rs::messages::boot_notification::BootNotificationRequest;
use ocpp_rs::structures::charging_station_type::ChargingStationType;
use ocpp_rs::structures::modem_type::ModemType;
use ocpp_rs::traits::OcppEntity;

use rootcause::prelude::*;

#[derive(Debug, Default)]
struct EVSEProperties {
    serial_number: String,
    make: String,
    model: String,
    firmware_version: String,
    modem: Option<ModemType>,
}

impl EVSEProperties {
    fn new(
        serial_number: String,
        make: String,
        model: String,
        firmware_version: String,
        modem: Option<ModemType>,
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
pub struct EVSE {
    properties: EVSEProperties,
}

impl Default for EVSE {
    fn default() -> Self {
        Self {
            properties: EVSEProperties::default(),
        }
    }
}

impl EVSE {
    /// Send an OCPP message to the CSMS
    pub async fn send_ocpp_message(&self, message: impl OcppEntity) -> Result<(), Report> {
        // TODO: Implement
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

    pub fn new() -> Self {
        Self {
            properties: Default::default(),
        }
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
    pub async fn boot(&self) -> Result<(), Report> {
        // Generate BootNotificationRequest
        let message = self.get_boot_notification()?;

        // Dispatch BNF
        self.send_ocpp_message(message.clone()).await?;

        // Await BootNotificationResponse from CSMS


        Ok(())
    }
}
