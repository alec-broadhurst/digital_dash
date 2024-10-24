use std::sync::{Arc, Condvar, Mutex};
use std::thread;

use iced::Task;

use gui::app::Dashboard;
use gui::utils::Telemetry;
use telemetry::config::Game;
use telemetry::games::forza::{ForzaParser, ForzaTelemetry};
use telemetry::parser::TelemetryParser;

mod gui;
mod telemetry;

fn main() -> iced::Result {
    let game: Game = Game::detect_game();
    let telemetry = match game {
        Game::Forza => Arc::new((Mutex::new(ForzaTelemetry::default()), Condvar::new())),
    };

    let telemetry_clone = telemetry.clone();
    thread::spawn(move || match game {
        Game::Forza => ForzaParser::parse_packets(telemetry_clone),
    });

    iced::application("Digital Dash", Dashboard::update, Dashboard::view)
        .run_with(|| (Dashboard::new(Telemetry::Forza(telemetry)), Task::none()))
}
