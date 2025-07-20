use crate::telemetry::telemetry_data::TelemetryData;

#[derive(Debug)]
pub enum Message {
    SwitchDashboard,
    UpdateTelemetry(Box<dyn TelemetryData>),
}
