use iced::widget::canvas::{Frame, Path};
use iced::{Color, Point};

pub trait Pin {
    fn path(&self, center: Point, gauge_radius: f32) -> Path;
    fn draw(&self, radius_for_frame: f32, frame: &mut Frame) {
        let dot = self.path(frame.center(), radius_for_frame);
        frame.fill(&dot, Color::WHITE);
    }
}

pub enum Pins {
    Small,
    Large,
}

impl Pin for Pins {
    fn path(&self, center: Point, gauge_radius: f32) -> Path {
        let v = match self {
            Pins::Small => 25.0,
            Pins::Large => 5.0,
        };
        Path::circle(center, gauge_radius / v)
    }
}
