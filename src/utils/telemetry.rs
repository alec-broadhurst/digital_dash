use crate::telemetry::games::forza::ForzaTelemetry;

#[derive(Default, Clone, Debug)]
pub enum Telemetry {
    #[default]
    None,
    Forza(ForzaTelemetry),
}
