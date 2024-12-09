use std::sync::{Arc, Condvar, Mutex};

use iced::{Element, Task};

use crate::gui::dashboards::forza_ui;
use crate::gui::message::Message;
use crate::gui::utils::DashboardVarient;
use crate::utils::telemetry::Telemetry;

#[derive(Default)]
pub struct Dashboard {
    telemetry: Arc<(Mutex<Telemetry>, Condvar)>,
    processed_telemetry: Telemetry,
    current_dashboard: DashboardVarient,
}

impl Dashboard {
    pub fn new(initial_telemetry: Arc<(Mutex<Telemetry>, Condvar)>) -> Self {
        Self {
            telemetry: initial_telemetry,
            processed_telemetry: Telemetry::default(),
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

            Message::ReadTelemetry => {
                let telemetry = self.telemetry.clone();
                Task::perform(Self::read_telemetry(telemetry), |output| output)
            }

            Message::UpdateTelemetry(telem) => {
                self.processed_telemetry = telem;
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        match self.current_dashboard {
            DashboardVarient::None => forza_ui::forza_dashboard(self),
            DashboardVarient::Forza => forza_ui::forza_dashboard(self),
        }
    }

    async fn read_telemetry(telemetry: Arc<(Mutex<Telemetry>, Condvar)>) -> Message {
        let (lock, cvar) = &*telemetry;
        let telem = cvar.wait(lock.lock().unwrap()).unwrap();
        let data = telem.clone();
        cvar.notify_one();
        Message::UpdateTelemetry(data)
    }

    pub fn get_telemetry(&self) -> &Telemetry {
        &self.processed_telemetry
    }
}
