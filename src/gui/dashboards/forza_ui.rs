use iced::widget::{container, text, Column, Container, Row, Text};
use iced::{Alignment, Color, Element, Length};

use crate::gui::app::Dashboard;
use crate::gui::message::Message;
use crate::gui::styles::container::ContainerType;
use crate::utils::telemetry::Telemetry;

pub fn forza_dashboard(dash: &Dashboard) -> Container<Message> {
    if let Some(Telemetry::Forza(forza_telemetry)) = dash.get_telemetry() {
        let mut body = Column::new()
            .width(Length::Fill)
            .padding(10)
            .spacing(10)
            .align_x(Alignment::Center);

        let gear = gear_container(forza_telemetry.get_gear());
        let speed = speed_container(forza_telemetry.get_speed());

        let tire_temps = tire_temps_container(forza_telemetry.get_tire_temps());

        body = body.push(speed).push(gear).push(tire_temps);

        Container::new(body)
            .width(Length::Fill)
            .height(Length::Fill)
            .center(Length::Fill)
            .style(ContainerType::rounded_border)
    } else {
        Container::new(Element::new(text!("Nothing here").size(75)))
            .center(Length::Fill)
            .style(ContainerType::rounded_border)
    }
}

fn speed_container(speed: f32) -> Element<'static, Message> {
    let text_color = Color::WHITE;

    container(text(format!("{}", speed)).size(50).color(text_color))
        .style(ContainerType::rounded_border)
        .into()
}

fn gear_container(gear: i32) -> Element<'static, Message> {
    let text_color = Color::WHITE;

    container(text(format!("{}", gear)).size(100).color(text_color))
        .style(ContainerType::rounded_border)
        .into()
}

fn tire_temps_container(tire_temps: (f32, f32, f32, f32)) -> Element<'static, Message> {
    let text_color = Color::WHITE;

    let front_temps = Row::new()
        .push(Text::new(format!("Front Left: {}", tire_temps.0)).color(text_color))
        .push(Text::new(format!("Front Right: {}", tire_temps.1)).color(text_color));

    let rear_temps = Row::new()
        .push(Text::new(format!("Front Left: {}", tire_temps.2)).color(text_color))
        .push(Text::new(format!("Front Right: {}", tire_temps.3)).color(text_color));

    container(Column::new().push(front_temps).push(rear_temps))
        .style(ContainerType::rounded_border)
        .into()
}
