use iced::widget::canvas::path::Builder;
use iced::widget::canvas::{stroke, Fill, Frame, LineCap, Path, Stroke};
use iced::{Color, Point, Vector};

pub trait Needle {
    fn path(&self, gauge_radius: f32) -> Path;
    fn tip(&self, gauge_radius: f32) -> Point {
        Point::new(0.5 * gauge_radius, 0.0)
    }
    fn stroke<'a>(&self, width: f32) -> Stroke<'a> {
        Stroke {
            width,
            style: stroke::Style::Solid(Color::BLACK),
            line_cap: LineCap::Round,
            ..Stroke::default()
        }
    }
    fn draw(&self, gauge_radius: f32, value: f32, frame: &mut Frame) {
        let tip = self.tip(gauge_radius);
        let path = self.path(gauge_radius);
        frame.fill(&path, Fill::default());
        frame.translate(Vector::new(tip.x, tip.y));
        frame.fill_text(format!("{}", value));
    }
}

pub enum Needles {
    Basic,
    Diamond,
}

impl Needle for Needles {
    fn path(&self, gauge_radius: f32) -> Path {
        match self {
            Needles::Basic => Path::line(Point::ORIGIN, self.tip(gauge_radius)),
            Needles::Diamond => {
                let mut b = Builder::new();
                b.move_to(Point::ORIGIN);
                b.line_to(Point::new(0.25 * gauge_radius, 7.5));
                b.line_to(self.tip(gauge_radius));
                b.line_to(Point::new(0.25 * gauge_radius, -7.5));
                b.line_to(Point::ORIGIN);
                b.close();

                b.build()
            }
        }
    }
}
