use crate::errors::EVSEError;
use miette::Result;
use std::collections::HashMap;

/// An enum with variants for each data a property may have.
#[derive(Debug, PartialEq)]
enum OcppProperty {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
    BoolArray(Vec<bool>),
    IntArray(Vec<i32>),
    FloatArray(Vec<f32>),
    StringArray(Vec<String>),
}

impl TryInto<bool> for OcppProperty {
    type Error = EVSEError;

    fn try_into(self) -> Result<bool, EVSEError> {
        if let OcppProperty::Bool(v) = self {
            return Ok(v);
        }

        Err(EVSEError::OcppPropertyError {
            t: String::from("bool"),
        })
    }
}

/// An enum that allows for Option-style checks against optional properties
#[derive(Debug, PartialEq, Default)]
enum OptionalProperty {
    Property(OcppProperty),

    #[default]
    None,
}

struct SystemProperties {
    csms_endpoint: String,
    ac_phase_switching_supported: bool, // If defined and true, this EVSE supports the selection of which phase to use for 1 phase AC charging
    active_monitoring_level: i32, // Shows the currently use MonitoringLevel.
    active_network_profile: bool, // Indicates the configuration profile the station uses to connect to the network
    active_transaction_id: String, // Active transaction on charging station or EVSE.
    additional_info_items_per_message: i32, // Maximum number of additionalInfo items that can be sent in one message.
    additional_root_certificate_check: bool, // When set to true, only one certificate (plus a temporary fallback certificate) of certificateType CSMSRootCertificate is allowed to be installed at a time.
    allow_energy_transfer_resumption: bool, // This variable defines whether energy transfer is allowed to be resumed when the transaction is resumed after a reset or power outage.
    allow_new_sessions_pending_firmware_update: bool, // Indicates whether new sessions can be started on EVSEs, while Charging Station is waiting for all EVSEs to become Available in order to start a pending firmware update.
    allow_rest: bool, // Component can be reset. Can be used to announce that an EVSE can be reset individually
    allow_security_profile_downgrade: bool, // If this variable is implemented and set to true, then the Charging Station allows downgrading the security profile from 3 to 2.
    authorize_remote_start: bool, // Whether a remote request to start a transaction in the form of RequestStartTransactionRequest message should be authorized beforehand like a local action to start a transaction.
}

#[derive(Debug)]
pub struct Evse {
    ocpp_properties: HashMap<String, OcppProperty>,
}

impl Default for Evse {
    fn default() -> Self {
        Self {
            ocpp_properties: HashMap::new(),
        }
    }
}

impl Evse {
    pub fn new() -> Self {
        Self {
            ocpp_properties: HashMap::new(),
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
    pub fn boot() -> Result<()> {
        // Generate BootNotificationRequest
        // Dispatch BNF
        // Await BootNotificationResponse from CSMS
        Ok(())
    }
}
