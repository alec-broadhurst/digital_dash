use iced::widget::text;
use iced::Element;

use crate::gui::app::Dashboard;
use crate::gui::message::Message;

pub fn forza_dashboard(dashboard: &Dashboard) -> Element<Message> {
    let telemetry = &dashboard.forza_telemetry;

    let rpm_lights = text!("**********************");
    Element::new(rpm_lights)
}
