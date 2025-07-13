use iced::widget::{text, Container};

use crate::gui::dashboard::Dashboard;
use crate::gui::message::Message;

pub fn no_dashboard(dash: &Dashboard) -> Container<Message> {
    if dash.get_telemetry().is_none() {
        Container::new(text!("No telemetry detected!"))
    } else {
        Container::new(text!("Nothing here"))
    }
}
