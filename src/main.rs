use gui::app::Dashboard;
use telemetry::config::Game;

mod gui;
mod telemetry;
mod utils;

fn main() -> iced::Result {
    let _game: Game = Game::detect_game();

    iced::application("Digital Dash", Dashboard::update, Dashboard::view)
        .subscription(Dashboard::subscription)
        .run_with(Dashboard::new)
}
