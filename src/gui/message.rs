use crate::utils::telemetry::Telemetry;

#[derive(Debug, Clone)]
pub enum Message {
    NoOp,
    SwitchDashboard,
    ReadTelemetry,
    UpdateTelemetry(Telemetry),
}
