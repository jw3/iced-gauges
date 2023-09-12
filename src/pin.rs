use crate::style::Appearance;
use crate::util::frame;
use iced::widget::canvas::{stroke, Frame, LineCap, Path, Stroke};

#[derive(Clone, Copy, Debug)]
pub enum PinOrder {
    Over,
    Under,
}

pub trait Pin {
    fn path(&self, frame: &Frame, style: &Appearance) -> Path {
        Path::circle(
            frame.center(),
            frame::radius(frame) * (style.pin_diameter_ratio / 4.0),
        )
    }
    fn stroke(&self, frame: &Frame, style: &Appearance) -> Stroke {
        Stroke {
            width: frame::radius(frame) * (style.pin_border_width_ratio / 4.0),
            style: stroke::Style::Solid(style.pin_border_color),
            line_cap: LineCap::Round,
            ..Stroke::default()
        }
    }
    fn draw(&self, frame: &mut Frame, style: &Appearance);
}

pub enum Pins {
    Solid,
    Hollow,
    Bordered,
}

impl Pin for Pins {
    fn draw(&self, frame: &mut Frame, style: &Appearance) {
        match self {
            Pins::Solid => frame.fill(&self.path(frame, style), style.pin_color),
            Pins::Hollow => frame.stroke(&self.path(frame, style), self.stroke(frame, style)),
            Pins::Bordered => {
                frame.fill(&self.path(frame, style), style.pin_color);
                frame.stroke(&self.path(frame, style), self.stroke(frame, style));
            }
        }
    }
}
