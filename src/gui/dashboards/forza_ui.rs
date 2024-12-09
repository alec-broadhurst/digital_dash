use iced::widget::text;
use iced::Element;

use crate::gui::app::Dashboard;
use crate::gui::message::Message;
use crate::utils::telemetry::Telemetry;

pub fn forza_dashboard(dashboard: &Dashboard) -> Element<Message> {
    let telemetry = dashboard.get_telemetry();
    if let Telemetry::Forza(telem) = telemetry {
        telem.get_gear();
    }
    let rpm_lights = text!("**********************");
    Element::new(rpm_lights)
}
