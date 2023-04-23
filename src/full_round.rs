use crate::Ellipse;
use iced::widget::canvas::{stroke, Cache, Cursor, Geometry, LineCap, Path, Program, Stroke};
use iced::{Color, Point, Rectangle, Theme, Vector};
use std::f32::consts::PI;

pub struct Gauge {
    value: f32,
    needle: Cache,
    frame: Cache,
    ticks: Cache,
}

impl Gauge {
    pub fn new() -> Self {
        Self {
            value: 0.0,
            needle: Default::default(),
            frame: Default::default(),
            ticks: Default::default(),
        }
    }

    pub fn update_value(&mut self, v: f32) {
        self.value = v;
        self.needle.clear();
    }
}

impl<T> Program<T> for Gauge {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let frame = self.frame.draw(bounds.size(), |frame| {
            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 2.5;

            let background = Path::circle(center, radius);
            frame.fill(&background, Color::from_rgb8(0x12, 0x93, 0xD8));
        });

        let needle = self.needle.draw(bounds.size(), |frame| {
            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 2.5;

            let width = radius / 100.0;
            let thin_stroke = |color: Color| -> Stroke {
                Stroke {
                    width,
                    style: stroke::Style::Solid(color),
                    line_cap: LineCap::Round,
                    ..Stroke::default()
                }
            };

            let length = PI * 2.0;
            let steps = length / 120.0;

            let short_hand = Path::line(Point::ORIGIN, Point::new(0.0, -0.5 * radius));
            frame.translate(Vector::new(center.x, center.y));

            frame.rotate(self.value as f32 * steps);
            frame.stroke(&short_hand, thin_stroke(Color::WHITE));

            frame.translate(Vector::new(0.0, -0.5 * radius));
            frame.fill_text(format!("{}", self.value));
        });

        let ticks = self.ticks.draw(bounds.size(), |frame| {
            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 2.5;
            let width = radius / 100.0;

            let thin_stroke = |color: Color| -> Stroke {
                Stroke {
                    width,
                    style: stroke::Style::Solid(color),
                    line_cap: LineCap::Round,
                    ..Stroke::default()
                }
            };

            frame.with_save(|frame| {
                frame.translate(Vector::new(center.x, center.y));

                let length = PI * 2.0;
                let step_12 = length / 12.0;

                frame.rotate(PI * 1.5);

                let outer = Ellipse {
                    center: Point::ORIGIN,
                    major_curvature: 1.0 / radius,
                    minor_radius: radius,
                    angle: 0.0,
                };

                let radius2 = radius - 25.0;
                let inner = Ellipse {
                    center: Point::ORIGIN,
                    major_curvature: 1.0 / radius2,
                    minor_radius: radius2,
                    angle: 0.0,
                };

                // hour ticks
                for a in 0..12 {
                    let angle = step_12 * a as f32;
                    let p1 = inner.get_point(angle);
                    let p2 = outer.get_point(angle);

                    let tick = Path::line(p1, p2);
                    frame.with_save(|frame| {
                        frame.stroke(&tick, thin_stroke(Color::BLACK));

                        frame.translate(Vector::new(p1.x, p1.y));
                        frame.fill_text(format!("{a}"));
                    });
                }
            });

            let width = radius / 100.0;
            let thin_stroke = |color: Color| -> Stroke {
                Stroke {
                    width,
                    style: stroke::Style::Solid(color),
                    line_cap: LineCap::Round,
                    ..Stroke::default()
                }
            };

            let dot = Path::circle(center, radius / 25.0);
            frame.fill(&dot, Color::BLACK);

            let background = Path::circle(center, radius);
            frame.stroke(&background, thin_stroke(Color::BLACK));
        });

        vec![frame, needle, ticks]
    }
}
