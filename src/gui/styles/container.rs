use iced::widget::container::Style;
use iced::{Background, Border, Color, Shadow, Theme};

pub struct ContainerType;

impl ContainerType {
    pub fn standard(_theme: &Theme) -> Style {
        Style {
            background: Some(Background::Color(Color::BLACK)),
            ..Default::default()
        }
    }

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

    pub fn rounded_filled(_theme: &Theme) -> Style {
        Style {
            text_color: Some(Color::BLACK),
            background: Some(Background::Color(Color::WHITE)),
            border: Border {
                color: Color::TRANSPARENT,
                width: 5.0,
                radius: 20.into(),
            },
            shadow: Shadow::default(),
        }
    }

    pub fn square_border(_theme: &Theme) -> Style {
        Style {
            text_color: Some(Color::WHITE),
            background: Some(Background::Color(Color::BLACK)),
            border: Border {
                color: Color::WHITE,
                width: 5.0,
                radius: 0.into(),
            },
            shadow: Shadow::default(),
        }
    }

    pub fn square_filled(_theme: &Theme) -> Style {
        Style {
            text_color: Some(Color::BLACK),
            background: Some(Background::Color(Color::WHITE)),
            border: Border {
                color: Color::TRANSPARENT,
                width: 5.0,
                radius: 0.into(),
            },
            shadow: Shadow::default(),
        }
    }
}
