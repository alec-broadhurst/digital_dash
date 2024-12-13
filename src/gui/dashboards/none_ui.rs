use iced::widget::text;
use iced::Element;

use crate::gui::app::Dashboard;
use crate::gui::message::Message;

pub fn no_dashboard(dash: &Dashboard) -> Element<Message> {
    if dash.get_telemetry().is_none() {
        Element::new(text!("No telemetry detected!"))
    } else {
        Element::new(text!("Nothing here"))
    }
}
