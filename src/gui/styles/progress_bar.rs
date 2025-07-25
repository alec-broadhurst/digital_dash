use iced::widget::progress_bar::Style;
use iced::{Background, Color, Theme};

pub struct ProgressBarStyle;

impl ProgressBarStyle {
    pub fn accel_bar(_theme: &Theme) -> Style {
        Style {
            background: Color::TRANSPARENT.into(),
            bar: Background::Color(Color::from_rgb8(0, 255, 0)),
            border: Default::default(),
        }
    }

    pub fn brake_bar(_theme: &Theme) -> Style {
        Style {
            background: Color::TRANSPARENT.into(),
            bar: Background::Color(Color::from_rgb8(255, 0, 0)),
            border: Default::default(),
        }
    }
}
