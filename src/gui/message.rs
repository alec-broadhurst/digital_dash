use crate::telemetry::games::forza::ForzaTelemetry;

#[derive(Debug, Clone)]
pub enum Message {
    NoOp,
    SwitchDashboard,
    UpdateTelemetry,
    UpdateForzaTelemetry(ForzaTelemetry),
}
