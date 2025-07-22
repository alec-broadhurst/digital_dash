use iced::widget::canvas::{self, Geometry};
use iced::{mouse, Color, Rectangle, Renderer, Theme};

pub struct RPMlight {
    radius: f32,
}

impl RPMlight {
    pub fn new(radius: f32) -> Self {
        RPMlight { radius }
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
        frame.fill(&rpm_light, Color::WHITE);

        vec![frame.into_geometry()]
    }
}
