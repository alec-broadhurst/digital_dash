use iced::futures::StreamExt;
use iced::{Element, Subscription, Task};

use crate::gui::dashboards::{forza_ui, none_ui};
use crate::gui::message::Message;
use crate::gui::utils::DashboardVarient;
use crate::telemetry::config::Game;
use crate::telemetry::games::forza::ForzaParser;
use crate::telemetry::parser::TelemetryParser;
use crate::telemetry::telemetry_data::TelemetryData;

pub struct Dashboard {
    telemetry: Option<Box<dyn TelemetryData>>,
    current_dashboard: DashboardVarient,
}

impl Dashboard {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                telemetry: None,
                current_dashboard: DashboardVarient::default(),
            },
            Task::future(async { Message::SwitchDashboard }),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SwitchDashboard => {
                self.current_dashboard = match Game::detect_game() {
                    Game::Forza => DashboardVarient::Forza,
                    Game::None => DashboardVarient::None,
                };
                Task::none()
            }

            Message::UpdateTelemetry(new_data) => {
                self.telemetry = Some(new_data);
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        match self.current_dashboard {
            DashboardVarient::None => none_ui::no_dashboard(self).into(),
            DashboardVarient::Forza => forza_ui::forza_dashboard(self).into(),
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        match self.current_dashboard {
            DashboardVarient::Forza => {
                Subscription::run(|| ForzaParser::parse_packets().map(Message::UpdateTelemetry))
            }

            _ => Subscription::none(),
        }
    }
}
