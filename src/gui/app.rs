use iced::{Element, Task};

use crate::gui::dashboards::forza_ui;
use crate::gui::message::Message;
use crate::gui::utils::DashboardVarient;
use crate::telemetry::games::forza::ForzaTelemetry;
use crate::utils::telemetry::Telemetry;

#[derive(Default)]
pub struct Dashboard {
    telemetry: Telemetry,
    pub forza_telemetry: ForzaTelemetry,
    current_dashboard: DashboardVarient,
}

impl Dashboard {
    pub fn new(initial_telemetry: Telemetry) -> Self {
        Self {
            telemetry: initial_telemetry,
            forza_telemetry: ForzaTelemetry::default(),
            current_dashboard: DashboardVarient::default(),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::NoOp => Task::none(),

            Message::SwitchDashboard => match self.current_dashboard {
                DashboardVarient::None => Task::none(),
                DashboardVarient::Forza => Task::none(),
            },

            Message::UpdateForzaTelemetry(telemetry) => {
                self.forza_telemetry = telemetry;
                Task::none()
            }

            Message::UpdateTelemetry => {
                let telemetry = self.telemetry.clone();
                Task::perform(Self::read_telemetry(telemetry), |output| output)
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        match self.current_dashboard {
            DashboardVarient::None => forza_ui::forza_dashboard(&self),
            DashboardVarient::Forza => forza_ui::forza_dashboard(&self),
        }
    }

    async fn read_telemetry(telemetry: Telemetry) -> Message {
        match telemetry {
            Telemetry::None => Message::NoOp,
            Telemetry::Forza(telemetry) => {
                let (lock, cvar) = &*telemetry;
                let data = cvar.wait(lock.lock().unwrap()).unwrap();
                Message::UpdateForzaTelemetry(data.clone())
            }
        }
    }
}
