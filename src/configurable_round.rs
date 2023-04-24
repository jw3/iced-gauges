use crate::{Ellipse, Ticks};
use iced::widget::canvas::{stroke, Cache, Cursor, Geometry, LineCap, Path, Program, Stroke};
use iced::{Color, Point, Rectangle, Theme, Vector};
use std::f32::consts::PI;

pub struct Gauge {
    value: f32,
    needle_gfx: Cache,
    bg_gfx: Cache,
    ticks_gfx: Cache,
    pin_gfx: Cache,
    length: f32,
    major_ticks: Ticks,
    minor_ticks: Ticks,
    min: f32,
    max: f32,
    step: f32,
    bg_color: Color,
    border_color: Color,
}

impl Gauge {
    pub fn new() -> Self {
        // provided
        let min = 0.0;
        let max = 60.0;
        let res = 1.0;

        // derived
        let cnt = (max - min) / res;
        let step = PI * 2.0 / cnt;

        Self {
            value: 0.0,
            needle_gfx: Default::default(),
            bg_gfx: Default::default(),
            ticks_gfx: Default::default(),
            pin_gfx: Default::default(),
            length: PI * 2.0,
            major_ticks: Ticks { first: 0, every: 5 },
            minor_ticks: Ticks { first: 0, every: 1 },
            min,
            max,
            step,
            bg_color: Color::from_rgb8(0x12, 0x93, 0xD8),
            border_color: Color::BLACK,
        }
    }

    pub fn update_value(&mut self, v: f32) {
        // todo;; what to do about constraining value by min/max?
        if v < self.min || v > self.max {
            println!("constratint violation: {} < {} < {}", self.min, v, self.max);
        }
        self.value = v;
        self.needle_gfx.clear();
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
        let bg = self.bg_gfx.draw(bounds.size(), |frame| {
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

            let background = Path::circle(center, radius);
            frame.fill(&background, self.bg_color);
            frame.stroke(&background, thin_stroke(self.border_color));
        });

        let needle = self.needle_gfx.draw(bounds.size(), |frame| {
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

            let short_hand = Path::line(Point::ORIGIN, Point::new(0.0, -0.5 * radius));
            frame.translate(Vector::new(center.x, center.y));

            frame.rotate(self.value as f32 * self.step);
            frame.stroke(&short_hand, thin_stroke(Color::WHITE));

            frame.translate(Vector::new(0.0, -0.5 * radius));
            frame.fill_text(format!("{}", self.value));
        });

        let ticks = self.ticks_gfx.draw(bounds.size(), |frame| {
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

                frame.rotate(PI * 1.5);

                let outer = Ellipse::round(radius);
                let major = Ellipse::round(radius - 25.0);
                let minor = Ellipse::round(radius - 5.0);

                let mut i = self.minor_ticks.first as f32;
                loop {
                    let angle = self.step * i;
                    let p1 = minor.get_point(angle);
                    let p2 = outer.get_point(angle);

                    let tick = Path::line(p1, p2);
                    frame.with_save(|frame| {
                        frame.stroke(&tick, thin_stroke(Color::WHITE));
                        frame.translate(Vector::new(p1.x, p1.y));
                    });

                    i += self.minor_ticks.every as f32;
                    if i * self.step >= self.length as f32 {
                        break;
                    }
                }

                let mut i = self.major_ticks.first as f32;
                loop {
                    let angle = self.step * i;
                    let p1 = major.get_point(angle);
                    let p2 = outer.get_point(angle);

                    let tick = Path::line(p1, p2);
                    frame.with_save(|frame| {
                        frame.stroke(&tick, thin_stroke(Color::BLACK));
                        frame.translate(Vector::new(p1.x, p1.y));
                        frame.fill_text(format!("{i}"));
                    });

                    i += self.major_ticks.every as f32;
                    if i * self.step >= self.length as f32 {
                        break;
                    }
                }
            });
        });

        let pin = self.pin_gfx.draw(bounds.size(), |frame| {
            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 2.5;
            let dot = Path::circle(center, radius / 25.0);
            frame.fill(&dot, Color::BLACK);
        });

        vec![bg, ticks, pin, needle]
    }
}
