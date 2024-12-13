use crate::utils::telemetry::Telemetry;
use iced::futures::Stream;

pub trait TelemetryParser {
    fn parse_packets() -> impl Stream<Item = Telemetry>;
}
