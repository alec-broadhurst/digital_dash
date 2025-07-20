use iced::widget::{container, progress_bar, text, Column, Container, Row, Text};
use iced::Length::{Fill, FillPortion};
use iced::{Alignment, Element};

use crate::gui::dashboard::Dashboard;
use crate::gui::message::Message;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::progress_bar::ProgressBarStyle;
use crate::utils::telemetry::Telemetry;

pub fn forza_dashboard(dash: &Dashboard) -> Container<Message> {
    if let Some(telemetry) = dash.get_telemetry() {
        // main column to hold all the widgets
        let mut dash = Column::new()
            .width(Fill)
            .padding(0)
            .spacing(10)
            .align_x(Alignment::Center);

        // top row to hold rpm lights
        let rpm_lights = rpm_lights_container();
        dash = dash.push(rpm_lights);

        // middle row to hold rpm, position, gear, speed, lap number, and time info
        let mut middle_row = Row::new().spacing(10);

        // left side of middle row
        let rpm_pos = container(
            Column::new()
                .push(rpm_container(telemetry.get_rpm()).center(FillPortion(1)))
                .push(position_container(telemetry.get_position()).center(FillPortion(1)))
                .push(empty_container().center(FillPortion(2)))
                .push(accel_container(telemetry.get_throttle()).center(FillPortion(1)))
                .push(brake_container(telemetry.get_brake()).center(FillPortion(1)))
                .push(empty_container().center(FillPortion(4)))
                .spacing(10),
        );

        middle_row = middle_row.push(rpm_pos);

        // center of middle row
        let speed_gear_temps = container(
            Column::new()
                .push(speed_container(telemetry.get_speed()).center(FillPortion(1)))
                .push(gear_container(telemetry.get_gear()).center(FillPortion(5)))
                .push(tire_temp_container(telemetry.get_tire_temps()).center(FillPortion(3)))
                .spacing(10),
        );

        middle_row = middle_row.push(speed_gear_temps);

        // right side of middle row
        let time_info = container(
            Column::new()
                .push(lap_num_container(telemetry.get_lap_number()).center(FillPortion(1)))
                .push(laptime_container(telemetry.get_lap_time()).center(FillPortion(3)))
                .push(
                    container(
                        Row::new()
                            .push(delta_container(telemetry.get_delta()))
                            .push(best_lap_container(telemetry.get_best_lap()))
                            .spacing(10),
                    )
                    .center(FillPortion(2)),
                )
                .push(empty_container().center(FillPortion(3)))
                .spacing(10),
        );

        middle_row = middle_row.push(time_info);

        dash = dash.push(middle_row);

        Container::new(dash)
            .width(Fill)
            .height(Fill)
            .center(Fill)
            .style(ContainerType::standard)
            .padding(40)
    } else {
        Container::new(Element::new(text!("Nothing here").size(75)))
            .center(Fill)
            .style(ContainerType::rounded_border)
    }
}

fn rpm_lights_container() -> Container<'static, Message> {
    container(text("TODO").size(50))
        .width(Fill)
        .style(ContainerType::rounded_filled)
        .center_x(Fill)
}

fn rpm_container(rpm: f32) -> Container<'static, Message> {
    container(Column::new().push(text(format!("{}", rpm)).size(30)))
        .style(ContainerType::rounded_border)
        .center_x(Fill)
        .padding(10)
}

fn position_container(pos: u8) -> Container<'static, Message> {
    container(
        Row::new()
            .push(
                container(Text::new("pos").size(30))
                    .style(ContainerType::square_filled)
                    .center(FillPortion(1)),
            )
            .push(container(Text::new(format!("{}", pos)).size(30)).center(FillPortion(2))),
    )
    .style(ContainerType::rounded_border)
    .center_x(Fill)
}

fn speed_container(speed: f32) -> Container<'static, Message> {
    container(Text::new(format!("{}", speed)).size(30))
        .center(Fill)
        .style(ContainerType::rounded_border)
}

fn gear_container(gear: u8) -> Container<'static, Message> {
    container(text(format!("{}", gear)).size(70))
        .center(Fill)
        .style(ContainerType::rounded_border)
}

fn lap_num_container(lap_num: u16) -> Container<'static, Message> {
    container(
        Row::new()
            .push(
                container(Text::new("Lap").size(30))
                    .style(ContainerType::square_filled)
                    .center(FillPortion(1)),
            )
            .push(container(Text::new(format!("{}", lap_num)).size(30)).center(FillPortion(2))),
    )
    .style(ContainerType::rounded_border)
    .center_x(Fill)
}

fn tire_temp_container(temps: (f32, f32, f32, f32)) -> Container<'static, Message> {
    let title = container(Row::new().push(Text::new("Tire Temps").size(30)))
        .style(ContainerType::square_filled)
        .center(Fill);
    let front_temps = Row::new()
        .push(container(Text::new(format!("{}", temps.0)).size(30)).center(FillPortion(1)))
        .push(container(Text::new(format!("{}", temps.1)).size(30)).center(FillPortion(1)));

    let rear_temps = Row::new()
        .push(container(Text::new(format!("{}", temps.2)).size(30)).center(FillPortion(1)))
        .push(container(Text::new(format!("{}", temps.3)).size(30)).center(FillPortion(1)));

    let all_temps = Column::new().push(title).push(front_temps).push(rear_temps);

    container(all_temps)
        .style(ContainerType::rounded_border)
        .center(Fill)
}

fn laptime_container(laptime: String) -> Container<'static, Message> {
    container(
        Column::new()
            .push(
                container(Text::new("Laptime").size(30))
                    .style(ContainerType::square_filled)
                    .center(FillPortion(1)),
            )
            .push(container(Text::new(laptime).size(40)).center(FillPortion(2))),
    )
    .style(ContainerType::rounded_border)
    .center_x(Fill)
}

fn delta_container(delta: String) -> Container<'static, Message> {
    container(
        Column::new()
            .push(
                container(Text::new("Delta").size(20))
                    .style(ContainerType::square_filled)
                    .center(FillPortion(1)),
            )
            .push(container(Text::new(delta).size(20)).center(FillPortion(2))),
    )
    .style(ContainerType::rounded_border)
    .center(Fill)
}

fn best_lap_container(laptime: String) -> Container<'static, Message> {
    container(
        Column::new()
            .push(
                container(Text::new("Best").size(20))
                    .style(ContainerType::square_filled)
                    .center(FillPortion(1)),
            )
            .push(container(Text::new(laptime).size(20)).center(FillPortion(2))),
    )
    .style(ContainerType::rounded_border)
    .center(Fill)
}

fn accel_container(accel: f32) -> Container<'static, Message> {
    container(progress_bar(0.0..=100.0, accel).style(ProgressBarStyle::accel_bar))
        .center(Fill)
        .style(ContainerType::rounded_border)
}

fn brake_container(brake: f32) -> Container<'static, Message> {
    container(progress_bar(0.0..=100.0, brake).style(ProgressBarStyle::brake_bar))
        .style(ContainerType::rounded_border)
}

fn empty_container() -> Container<'static, Message> {
    container(Text::new("").size(50))
        .center(Fill)
        .style(ContainerType::standard)
}
