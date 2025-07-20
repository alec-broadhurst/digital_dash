use std::{any::Any, fmt::Debug};

// === Trait for generic telemetry data ===
pub trait TelemetryData: Any + Send + Debug {
    fn get_speed(&self) -> f32;
    fn get_rpm(&self) -> f32;
    fn get_gear(&self) -> u8;
    fn get_throttle(&self) -> f32;
    fn get_brake(&self) -> f32;
    fn get_position(&self) -> u8;
    fn get_lap_number(&self) -> u16;
    fn get_lap_time(&self) -> String;
    fn get_best_lap(&self) -> String;
    fn get_tire_temps(&self) -> (f32, f32, f32, f32);
    fn get_delta(&self) -> String;
}
