use std::sync::{Arc, Condvar, Mutex};

use crate::utils::telemetry::Telemetry;

pub trait TelemetryParser {
    fn parse_packets(telemetry: Arc<(Mutex<Telemetry>, Condvar)>);
}
