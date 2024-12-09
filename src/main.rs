use std::sync::{Arc, Condvar, Mutex};
use std::thread;

use iced::Task;

use gui::app::Dashboard;
use telemetry::config::Game;
use telemetry::games::forza::{ForzaParser, ForzaTelemetry};
use telemetry::parser::TelemetryParser;
use utils::telemetry::Telemetry;

mod gui;
mod telemetry;
mod utils;

fn main() -> iced::Result {
    let game: Game = Game::detect_game();
    let telemetry = match game {
        Game::Forza => Arc::new((
            Mutex::new(Telemetry::Forza(ForzaTelemetry::default())),
            Condvar::new(),
        )),
    };

    let telemetry_clone = telemetry.clone();
    thread::spawn(move || match game {
        Game::Forza => ForzaParser::parse_packets(telemetry_clone),
    });

    iced::application("Digital Dash", Dashboard::update, Dashboard::view)
        .run_with(|| (Dashboard::new(telemetry), Task::none()))
}
