use iced::widget::canvas::path::Builder;
use iced::widget::canvas::{stroke, Fill, Frame, LineCap, Path, Stroke};
use iced::{Color, Point};

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
    fn draw(&self, gauge_radius: f32, _value: f32, frame: &mut Frame) {
        let path = self.path(gauge_radius);
        frame.fill(&path, Fill::default());
        // let tip = self.tip(gauge_radius);
        // frame.translate(Vector::new(tip.x, tip.y));
        // frame.fill_text(format!("{}", value));
    }
}

pub enum Needles {
    Basic,
    Diamond,
    Arrow,
    Triangle,
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
            Needles::Arrow => {
                let inset = gauge_radius / 10.0;
                let rear_width = 0.25 * gauge_radius / 2.0;
                let tail = Point::new(inset, 0.0);
                let mut b = Builder::new();
                b.move_to(tail);
                b.line_to(Point::new(10.0, rear_width));
                b.line_to(self.tip(gauge_radius));
                b.line_to(Point::new(10.0, -rear_width));
                b.line_to(tail);
                b.close();

                b.build()
            }
            Needles::Triangle => {
                let rear_length = gauge_radius / 5.0;
                let rear_width = 0.25 * gauge_radius / 5.0;
                let tail = Point::new(-rear_length, 0.0);
                let mut b = Builder::new();
                b.move_to(tail);
                b.line_to(Point::new(tail.x, rear_width));
                b.line_to(self.tip(gauge_radius));
                b.line_to(Point::new(tail.x, -rear_width));
                b.line_to(tail);
                b.close();

                b.build()
            }
        }
    }
}
