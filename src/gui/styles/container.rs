use iced::widget::container::Style;
use iced::{Background, Border, Color, Shadow, Theme};

pub struct ContainerType;

impl ContainerType {
    pub fn rounded_border(_theme: &Theme) -> Style {
        Style {
            text_color: Some(Color::WHITE),
            background: Some(Background::Color(Color::BLACK)),
            border: Border {
                color: Color::WHITE,
                width: 5.0,
                radius: 20.into(),
            },
            shadow: Shadow::default(),
        }
    }
}
