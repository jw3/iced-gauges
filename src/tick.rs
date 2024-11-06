use iced::widget::canvas::path::arc::Elliptical;
use iced::widget::canvas::path::Builder;
use iced::widget::canvas::{stroke, Frame, LineCap, Path, Stroke, Text};
use iced::{Color, Point, Radians, Vector};

use crate::style::Appearance;
use crate::util::frame;
use crate::Ellipse;

pub trait Tick {
    /// Draw the ticks onto the frame
    /// The gauge radius is defined in pixels
    /// The gauge length defines the radians of needle movement
    /// The step length defines the radians of movement per unit value
    fn draw(
        &self,
        frame: &mut Frame,
        style: &Appearance,
        gauge_length: Radians,
        step_length: Radians,
    );
}

#[derive(Copy, Clone, Debug)]
pub struct MajorMinor {
    /// radian position of the first tick
    pub first: f32,
    /// the number of units of measure (uom) between ticks
    pub minor_step: f32,
    pub major_step: f32,
    /// length of stroke, as ratio of radius 0.0 -- 1.0
    pub major_length: f32,
    pub minor_length: f32,
}

impl MajorMinor {
    pub fn boxed(first: f32, major_step: f32, minor_step: f32, length: f32) -> Box<Self> {
        Box::new(MajorMinor {
            first,
            major_step,
            minor_step,
            major_length: length,
            minor_length: length * 0.75,
        })
    }
}

fn stroke<'a>(width: f32, color: Color) -> Stroke<'a> {
    Stroke {
        width,
        style: stroke::Style::Solid(color),
        line_cap: LineCap::Round,
        ..Stroke::default()
    }
}

impl Tick for MajorMinor {
    fn draw(&self, frame: &mut Frame, style: &Appearance, size: Radians, step: Radians) {
        let mut i = self.first;
        let radius = frame::radius(frame) * style.tick_border_inset_ratio;

        let major = Ellipse::round(radius - radius * self.major_length);
        let minor = Ellipse::round(radius - radius * self.minor_length);
        let outer = Ellipse::round(radius);

        let width_ratio = radius / 100.0;

        loop {
            match (i % self.major_step == 0.0, i % self.minor_step == 0.0) {
                (true, _) => {
                    let angle = i * step;
                    let p1 = major.get_point(angle.0);
                    let p2 = outer.get_point(angle.0);
                    let path = Path::line(p1, p2);

                    frame.with_save(|frame| {
                        frame.stroke(
                            &path,
                            stroke(
                                width_ratio * style.major_tick_width_ratio,
                                style.major_tick_color,
                            ),
                        );
                        frame.translate(Vector::new(p1.x, p1.y));
                        if style.tick_labels {
                            frame.fill_text(Text {
                                content: i.to_string(),
                                color: style.tick_text_color,
                                ..Text::default()
                            });
                        }
                    });
                }
                (_, true) => {
                    let angle = i * step;
                    let p1 = minor.get_point(angle.0);
                    let p2 = outer.get_point(angle.0);
                    let path = Path::line(p1, p2);

                    frame.with_save(|frame| {
                        frame.stroke(
                            &path,
                            stroke(
                                width_ratio * style.minor_tick_width_ratio,
                                style.minor_tick_color,
                            ),
                        );
                        frame.translate(Vector::new(p1.x, p1.y));
                    });
                }
                _ => {}
            }

            if i * step >= size {
                break;
            }

            i += 1.0;
        }

        frame.with_save(|frame| {
            let mut builder = Builder::new();
            builder.ellipse(Elliptical {
                center: Point::ORIGIN,
                radii: Vector::new(radius, radius),
                rotation: Radians(0.0),
                start_angle: Radians(self.first * step.0),
                end_angle: Radians(size.0),
            });
            let out = builder.build();
            frame.stroke(
                &out,
                stroke(width_ratio * style.tick_border_width_ratio, Color::BLACK),
            );
        });
    }
}
