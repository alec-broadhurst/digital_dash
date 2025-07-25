use iced::futures::{SinkExt, Stream};
use iced::stream;

use crate::telemetry::parser::TelemetryParser;
use crate::telemetry::telemetry_data::TelemetryData;
use crate::telemetry::utils::{parse_f32_from_bytes, parse_u16_from_bytes, setup_udp_socket};

#[derive(Debug, Clone)]
pub struct ForzaTelemetry {
    pub current_rpm: f32,
    pub max_rpm: f32,
    pub speed: f32,
    pub best_lap: f32,
    pub prev_best: f32,
    pub current_lap: f32,
    pub last_lap: f32,
    pub gear: u8,
    pub throttle: f32,
    pub brake: f32,
    pub position: u8,
    pub temp_left_f: f32,
    pub temp_right_f: f32,
    pub temp_left_r: f32,
    pub temp_right_r: f32,
    pub lap_number: u16,
}

impl TelemetryData for ForzaTelemetry {
    fn get_speed(&self) -> f32 {
        self.speed
    }
    fn get_rpm(&self) -> f32 {
        self.current_rpm
    }

    fn get_max_rpm(&self) -> f32 {
        self.max_rpm
    }

    fn get_gear(&self) -> u8 {
        self.gear
    }
    fn get_throttle(&self) -> f32 {
        self.throttle
    }
    fn get_brake(&self) -> f32 {
        self.brake
    }
    fn get_position(&self) -> u8 {
        self.position
    }
    fn get_lap_number(&self) -> u16 {
        self.lap_number
    }
    fn get_lap_time(&self) -> String {
        Self::format_time(self.current_lap)
    }

    fn get_best_lap(&self) -> String {
        Self::format_time(self.best_lap)
    }

    fn get_tire_temps(&self) -> (f32, f32, f32, f32) {
        (
            self.temp_left_f,
            self.temp_right_f,
            self.temp_left_r,
            self.temp_right_r,
        )
    }

    fn get_delta(&self) -> String {
        let delta = if self.last_lap == self.best_lap {
            self.last_lap - self.prev_best
        } else {
            self.last_lap - self.best_lap
        };

        Self::format_time(delta)
    }
}

impl ForzaTelemetry {
    fn format_time(time: f32) -> String {
        let minutes = (time.abs() / 60.0).floor() as i32;
        let seconds = (time.abs() % 60.0).floor() as i32;
        let milliseconds = (time.abs() * 1000.0).round() as i32 % 1000;

        if time < 0.0 {
            format!("-{:02}:{:02}.{:03}", minutes, seconds, milliseconds)
        } else {
            format!("{:02}:{:02}.{:03}", minutes, seconds, milliseconds)
        }
    }

    fn from_packet(buf: &[u8], prev_best: f32) -> Self {
        ForzaTelemetry {
            current_rpm: parse_f32_from_bytes(&buf[16..20]).round(),
            max_rpm: parse_f32_from_bytes(&buf[8..12]),
            speed: (parse_f32_from_bytes(&buf[244..248]) * 2.237).round(),
            best_lap: parse_f32_from_bytes(&buf[284..288]),
            current_lap: parse_f32_from_bytes(&buf[292..296]),
            last_lap: parse_f32_from_bytes(&buf[288..292]),
            lap_number: parse_u16_from_bytes(&buf[300..302]),
            position: buf[302],
            gear: buf[307],
            throttle: buf[303] as f32,
            brake: buf[304] as f32,
            temp_left_f: parse_f32_from_bytes(&buf[256..260]).round(),
            temp_right_f: parse_f32_from_bytes(&buf[260..264]).round(),
            temp_left_r: parse_f32_from_bytes(&buf[264..268]).round(),
            temp_right_r: parse_f32_from_bytes(&buf[268..272]).round(),
            prev_best,
        }
    }
}

pub struct ForzaParser;

impl TelemetryParser for ForzaParser {
    fn parse_packets() -> impl Stream<Item = Box<dyn TelemetryData>> {
        stream::channel(1, |mut output| async move {
            let socket = setup_udp_socket();
            socket.set_nonblocking(true).unwrap();

            let mut prev_best = 0.0;
            let mut buf = vec![0u8; 500];

            loop {
                if socket.recv(&mut buf).is_ok() {
                    let telemetry = ForzaTelemetry::from_packet(&buf, prev_best);

                    if telemetry.last_lap != telemetry.best_lap {
                        prev_best = telemetry.best_lap;
                    }

                    if output
                        .send(Box::new(telemetry) as Box<dyn TelemetryData>)
                        .await
                        .is_err()
                    {
                        break;
                    }
                }
            }
        })
    }
}
