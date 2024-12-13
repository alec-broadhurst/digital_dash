use iced::widget::{container, text, Column, Container, Row, Text};
use iced::{theme, Alignment, Element};

use crate::gui::app::Dashboard;
use crate::gui::message::Message;
use crate::utils::telemetry::Telemetry;

pub fn forza_dashboard(dash: &Dashboard) -> Element<Message> {
    if let Some(Telemetry::Forza(forza_telemetry)) = dash.get_telemetry() {
        let gear = Text::new(forza_telemetry.get_gear());
        Element::new(gear)
    } else {
        Element::new(text!("Nothing here"))
    }
}
