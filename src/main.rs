mod gui;
mod telemetry;

use gui::dashboard::Dashboard;

fn main() -> iced::Result {
    iced::application("Digital Dash", Dashboard::update, Dashboard::view)
        .subscription(Dashboard::subscription)
        .run_with(Dashboard::new)
}
