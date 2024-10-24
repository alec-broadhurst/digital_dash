use std::sync::{Arc, Condvar, Mutex};

use crate::telemetry::games::forza::ForzaTelemetry;

#[derive(Default, Clone)]
pub enum Telemetry {
    #[default]
    None,
    Forza(Arc<(Mutex<ForzaTelemetry>, Condvar)>),
}
