use crate::utils::telemetry::Telemetry;

#[derive(Debug, Clone)]
pub enum Message {
    SwitchDashboard,
    UpdateTelemetry(Telemetry),
}
