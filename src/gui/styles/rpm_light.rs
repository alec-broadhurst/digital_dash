use iced::widget::canvas::{self, Geometry};
use iced::{mouse, Color, Rectangle, Renderer, Theme};

pub struct RPMlight {
    radius: f32,
    activation_threshold: f32,
    is_active: bool,
    activated_color: Color,
    deactivated_color: Color,
}

impl RPMlight {
    pub fn new(
        radius: f32,
        activation_threshold: f32,
        is_active: bool,
        activated_color: Color,
        deactivated_color: Color,
    ) -> Self {
        RPMlight {
            radius,
            activation_threshold,
            is_active,
            activated_color,
            deactivated_color,
        }
    }
}

impl<Message> canvas::Program<Message> for RPMlight {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let rpm_light = canvas::Path::circle(frame.center(), self.radius);
        frame.fill(
            &rpm_light,
            if self.is_active {
                self.activated_color
            } else {
                self.deactivated_color
            },
        );

        vec![frame.into_geometry()]
    }
}
