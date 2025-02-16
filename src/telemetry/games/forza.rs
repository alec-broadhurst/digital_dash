use std::net::UdpSocket;

use iced::futures::{SinkExt, Stream};
use iced::stream;

use crate::telemetry::parser::TelemetryParser;
use crate::telemetry::utils::{parse_f32_from_bytes, parse_u16_from_bytes, setup_udp_socket};
use crate::utils::telemetry::Telemetry;

#[derive(Default, Clone, Debug)]
pub struct ForzaTelemetry {
    current_rpm: f32,
    max_rpm: f32,
    speed: f32,
    best_lap: f32,
    prev_best: f32,
    current_lap: f32,
    last_lap: f32,
    gear: u8,
    accel: f32,
    brake: f32,
    position: u8,
    temp_left_f: f32,
    temp_right_f: f32,
    temp_left_r: f32,
    temp_right_r: f32,
    lap_number: u16,
}

impl ForzaTelemetry {
    pub fn get_current_rpm(&self) -> f32 {
        self.current_rpm
    }

    pub fn get_max_rpm(&self) -> f32 {
        self.max_rpm
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    pub fn get_best_lap(&self) -> String {
        Self::format_time(self.best_lap)
    }

    pub fn get_current_lap(&self) -> String {
        Self::format_time(self.current_lap)
    }

    pub fn get_gear(&self) -> u8 {
        self.gear
    }

    pub fn get_accel(&self) -> f32 {
        self.accel / 255.0 * 100.0
    }

    pub fn get_brake(&self) -> f32 {
        self.brake / 255.0 * 100.0
    }

    pub fn get_position(&self) -> u8 {
        self.position
    }

    pub fn get_temp_left_f(&self) -> f32 {
        self.temp_left_f
    }

    pub fn get_temp_right_f(&self) -> f32 {
        self.temp_right_f
    }

    pub fn get_temp_left_r(&self) -> f32 {
        self.temp_left_r
    }

    pub fn get_temp_right_r(&self) -> f32 {
        self.temp_right_r
    }

    pub fn get_tire_temps(&self) -> (f32, f32, f32, f32) {
        (
            self.temp_left_f,
            self.temp_right_f,
            self.temp_left_r,
            self.temp_right_r,
        )
    }

    pub fn get_lap_number(&self) -> u16 {
        self.lap_number + 1
    }

    pub fn get_delta(&self) -> String {
        let delta = if self.last_lap == self.best_lap {
            self.last_lap - self.prev_best
        } else {
            self.last_lap - self.best_lap
        };

        Self::format_time(delta)
    }

    fn format_time(time: f32) -> String {
        let minutes: i32 = (time.abs() / 60.0).floor() as i32;
        let seconds: i32 = (time.abs() % 60.0).floor() as i32;
        let milliseconds: i32 = (time.abs() * 1000.0).round() as i32 % 1000;

        if time < 0.0 {
            format!("-{:02}:{:02}.{:03}", minutes, seconds, milliseconds)
        } else {
            format!("{:02}:{:02}.{:03}", minutes, seconds, milliseconds)
        }
    }
}

pub struct ForzaParser;

impl TelemetryParser for ForzaParser {
    fn parse_packets() -> impl Stream<Item = Telemetry> {
        stream::channel(1, |mut output| async move {
            let socket: UdpSocket = setup_udp_socket();
            socket.set_nonblocking(true).unwrap();

            let mut prev_state: Option<ForzaTelemetry> = None;
            let mut buf: Vec<u8> = vec![0; 500];

            loop {
                if socket.recv(&mut buf).is_ok() {
                    let mut current = ForzaTelemetry {
                        current_rpm: parse_f32_from_bytes(&buf[16..20]).round(),
                        max_rpm: parse_f32_from_bytes(&buf[8..12]),
                        speed: (parse_f32_from_bytes(&buf[244..248]) * 2.237).round(),
                        best_lap: parse_f32_from_bytes(&buf[284..288]),
                        current_lap: parse_f32_from_bytes(&buf[292..296]),
                        last_lap: parse_f32_from_bytes(&buf[288..292]),
                        lap_number: parse_u16_from_bytes(&buf[300..302]),
                        position: buf[302],
                        gear: buf[307],
                        accel: buf[303] as f32,
                        brake: buf[304] as f32,
                        temp_left_f: parse_f32_from_bytes(&buf[256..260]).round(),
                        temp_right_f: parse_f32_from_bytes(&buf[260..264]).round(),
                        temp_left_r: parse_f32_from_bytes(&buf[264..268]).round(),
                        temp_right_r: parse_f32_from_bytes(&buf[268..272]).round(),
                        prev_best: 0.0,
                    };

                    if let Some(prev) = &prev_state {
                        if prev.prev_best == 0.0 && current.lap_number > 1 {
                            current.prev_best = current.best_lap;
                        }

                        if current.last_lap != current.best_lap {
                            current.prev_best = current.best_lap;
                        }
                    }

                    if output
                        .send(Telemetry::Forza(current.clone()))
                        .await
                        .is_err()
                    {
                        break;
                    }

                    prev_state = Some(current);
                }
            }
        })
    }
}
