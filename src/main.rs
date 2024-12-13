use gui::app::Dashboard;

mod gui;
mod telemetry;
mod utils;

fn main() -> iced::Result {
    iced::application("Digital Dash", Dashboard::update, Dashboard::view)
        .subscription(Dashboard::subscription)
        .run_with(Dashboard::new)
}
