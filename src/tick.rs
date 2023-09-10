use crate::Ellipse;
use iced::widget::canvas::path::arc::Elliptical;
use iced::widget::canvas::path::Builder;
use iced::widget::canvas::{stroke, Frame, LineCap, Path, Stroke};
use iced::{Color, Point, Vector};

pub trait Tick {
    fn path(&self, gauge_radius: f32) -> Path;
    fn stroke<'a>(&self, width: f32, color: Color) -> Stroke<'a>;
    fn draw(&self, frame: &mut Frame, gauge_radius: f32, size: f32, step: f32, rotate: f32);
}

#[derive(Copy, Clone, Debug)]
pub struct DefaultTick {
    /// radian position of the first tick
    pub first: f32,
    /// the number of units of measure (uom) between ticks
    pub minor_step: f32,
    pub major_step: f32,
    /// length of stroke, as ratio of radius 0.0 -- 1.0
    pub major_length: f32,
    pub minor_length: f32,
    /// width of stroke
    pub width: f32,
    pub label: bool,
    pub major_color: Color,
    pub minor_color: Color,
}

impl DefaultTick {
    pub fn boxed(
        first: f32,
        major_step: f32,
        minor_step: f32,
        length: f32,
        width: f32,
        label: bool,
        major_color: Color,
        minor_color: Color,
    ) -> Self {
        DefaultTick {
            first,
            major_step,
            minor_step,
            major_length: length,
            minor_length: length * 0.75,
            width,
            label,
            major_color,
            minor_color,
        }
    }
}

impl Tick for DefaultTick {
    fn path(&self, gauge_radius: f32) -> Path {
        todo!()
    }

    fn stroke<'a>(&self, width: f32, color: Color) -> Stroke<'a> {
        Stroke {
            width,
            style: stroke::Style::Solid(color),
            line_cap: LineCap::Round,
            ..Stroke::default()
        }
    }

    fn draw(&self, frame: &mut Frame, radius: f32, size: f32, step: f32, rotate: f32) {
        let mut i = self.first;
        let mut current_step = 0;

        let width = radius / 100.0;
        let major = Ellipse::round(radius - radius * self.major_length);
        let minor = Ellipse::round(radius - radius * self.minor_length);
        let outer = Ellipse::round(radius);

        loop {
            match (i % self.major_step == 0.0, i % self.minor_step == 0.0) {
                (true, _) => {
                    let angle = i * step;
                    let p1 = major.get_point(angle);
                    let p2 = outer.get_point(angle);
                    let path = Path::line(p1, p2);

                    frame.with_save(|frame| {
                        frame.stroke(&path, self.stroke(width * self.width, self.major_color));
                        frame.translate(Vector::new(p1.x, p1.y));
                        if self.label {
                            frame.fill_text(format!("{i}"));
                        }
                    });
                }
                (_, true) => {
                    let angle = i * step;
                    let p1 = minor.get_point(angle);
                    let p2 = outer.get_point(angle);
                    let path = Path::line(p1, p2);

                    frame.with_save(|frame| {
                        frame.stroke(&path, self.stroke(width * self.width, self.minor_color));
                        frame.translate(Vector::new(p1.x, p1.y));
                    });
                }
                _ => {}
            }

            if i * step >= size {
                break;
            }

            i += 1.0;
            current_step += 1;
        }

        frame.with_save(|frame| {
            let mut builder = Builder::new();
            builder.ellipse(Elliptical {
                center: Point::ORIGIN,
                radii: Vector::new(radius, radius),
                rotation: 0.0,
                start_angle: self.first * step,
                end_angle: size,
            });
            let out = builder.build();
            frame.stroke(&out, self.stroke(width * self.width, Color::BLACK));
        });
    }
}
