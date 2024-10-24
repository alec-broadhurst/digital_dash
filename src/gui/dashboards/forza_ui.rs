use iced::widget::text;
use iced::Element;

use crate::gui::app::Dashboard;
use crate::gui::utils::Message;

pub fn forza_dashboard(dashboard: &Dashboard) -> Element<Message> {
    let rpm_lights = text!("**********************");
    Element::new(rpm_lights)
}
