use std::net::UdpSocket;
use std::sync::mpsc::Sender;
use std::vec;
use local_ip_address::local_ip;


pub struct PacketInfo {
    current_rpm: f32,
    max_rpm: f32,
    speed: f32,
    best_lap: f32,
    current_lap: f32,
    current_race_time: f32,
    gear: i32,
    accel: f32,
    brake: f32,
    position: i32,
    temp_left_f: f32,
    temp_right_f: f32,
    temp_left_r: f32,
    temp_right_r: f32,
    lap_number: i32
}

impl PacketInfo {
    pub fn get_current_rpm(&self) -> f32 {
        self.current_rpm
    }

    pub fn get_max_rpm(&self) -> f32 {
        self.max_rpm
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    pub fn get_best_lap(&self) -> f32 {
        self.best_lap
    }

    pub fn get_current_lap(&self) -> f32 {
        self.current_lap
    }

    pub fn get_current_race_time(&self) -> f32 {
        self.current_race_time
    }

    pub fn get_gear(&self) -> i32 {
        self.gear
    }

    pub fn get_accel(&self) -> f32 {
        self.accel / 255.0 * 100.0
    }

    pub fn get_brake(&self) -> f32 {
        self.brake / 255.0 * 100.0
    }

    pub fn get_position(&self) -> i32 {
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

    pub fn get_lap_number(&self) -> i32 {
        self.lap_number + 1
    }
}


fn setup_udp_socket() -> UdpSocket {
    let ip_addr: String = local_ip().unwrap().to_string();
    let port: &str = "8080";
    let binding_addr: String = format!("{}:{}", ip_addr, port);
    let socket: UdpSocket = UdpSocket::bind(binding_addr).expect("Failed to bind to address");

    socket
}

fn parse_f32_from_bytes(buf: &[u8]) -> f32 {
    f32::from_le_bytes(buf.try_into().expect("Failed to convert bytes to f32"))
}

fn parse_i32_from_bytes(buf: &[u8]) -> i32 {
    i32::from_le_bytes(buf.try_into().expect("Failed to convert bytes to i32"))
}

pub fn parse_packets(sender: Sender<PacketInfo>) {
    let socket: UdpSocket = setup_udp_socket();
    let mut buf: Vec<u8> = vec![0; 500];

    loop {
        socket.recv_from(&mut buf).expect("Failed to receive data");

        let packet_info: PacketInfo = PacketInfo {
            current_rpm: parse_f32_from_bytes(&buf[16..20]).round(),
            max_rpm: parse_f32_from_bytes(&buf[8..12]),
            speed: (parse_f32_from_bytes(&buf[244..248]) * 2.237).round(),
            best_lap: parse_f32_from_bytes(&buf[284..288]),
            current_lap: parse_f32_from_bytes(&buf[292..296]),
            current_race_time: parse_f32_from_bytes(&buf[296..300]),
            lap_number: parse_i32_from_bytes(&buf[300..302]),
            position: buf[302] as i32,
            gear: buf[307] as i32,
            accel: buf[303] as f32,
            brake: buf[304] as f32,
            temp_left_f: parse_f32_from_bytes(&buf[256..260]).round(),
            temp_right_f: parse_f32_from_bytes(&buf[260..264]).round(),
            temp_left_r: parse_f32_from_bytes(&buf[264..268]).round(),
            temp_right_r: parse_f32_from_bytes(&buf[268..272]).round(),
        };

        sender.send(packet_info).expect("Error sending packet data to thread");
    }
}