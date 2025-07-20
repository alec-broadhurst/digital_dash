use crate::telemetry::telemetry_data::TelemetryData;
use iced::futures::Stream;

pub trait TelemetryParser {
    fn parse_packets() -> impl Stream<Item = Box<dyn TelemetryData>>;
}
