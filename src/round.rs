use crate::{Ellipse, Ticks};
use iced::widget::canvas::path::arc::Elliptical;
use iced::widget::canvas::path::Builder;
use iced::widget::canvas::{stroke, Cache, Cursor, Geometry, LineCap, Path, Program, Stroke};
use iced::{Color, Point, Rectangle, Theme, Vector};
use std::f32::consts::{PI, TAU};

pub struct Gauge {
    value: f32,
    needle_gfx: Cache,
    bg_gfx: Cache,
    ticks_gfx: Cache,
    pin_gfx: Cache,
    dbg_gfx: Cache,
    length: f32,
    rotate: f32,
    major_ticks: Ticks,
    minor_ticks: Ticks,
    min: f32,
    max: f32,
    step: f32,
    bg_color: Color,
    border_color: Color,
}

impl Gauge {
    // length and rotate are fractions of 1.0 which is a full circle
    // the value of these can exceed 1.0, in which case it loops
    pub fn new(min: f32, max: f32, length: f32, rotate: f32) -> Self {
        // wait for builder impl
        let res = 1.0; // resolution: ie. visible values

        // derived
        let rotate = TAU * rotate;
        let length = TAU * length;
        let cnt = (max - min) / res;
        let step = length / cnt;

        Self {
            value: 0.0,
            needle_gfx: Default::default(),
            bg_gfx: Default::default(),
            ticks_gfx: Default::default(),
            pin_gfx: Default::default(),
            dbg_gfx: Default::default(),
            length,
            rotate,
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

            let mut builder = Builder::new();
            builder.ellipse(Elliptical {
                center,
                radii: Vector::new(radius, radius),
                rotation: self.rotate,
                start_angle: 0.0,
                end_angle: self.length,
            });
            // todo; =========== dont forget about this one ===============
            builder.line_to(center);
            // ============================================================
            builder.close();

            let background = builder.build();
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

            frame.with_save(|frame| {
                frame.translate(Vector::new(center.x, center.y));

                let tip = Point::new(0.5 * radius, 0.0);
                let short_hand = Path::line(Point::ORIGIN, tip);
                frame.rotate(self.rotate);

                frame.rotate(self.value as f32 * self.step);
                frame.stroke(&short_hand, thin_stroke(Color::BLACK));

                frame.translate(Vector::new(tip.x, tip.y));
                frame.fill_text(format!("{}", self.value));
            });
        });

        let ticks = self.ticks_gfx.draw(bounds.size(), |frame| {
            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 2.5;
            let width = radius / 100.0;

            let stroke = |w: f32, color: Color| -> Stroke {
                Stroke {
                    width: w,
                    style: stroke::Style::Solid(color),
                    line_cap: LineCap::Square,
                    ..Stroke::default()
                }
            };

            frame.with_save(|frame| {
                frame.translate(Vector::new(center.x, center.y));

                frame.rotate(self.rotate);

                let outer = Ellipse::round(radius);
                let major = Ellipse::round(radius - 30.0);
                let minor = Ellipse::round(radius - 10.0);

                let mut i = self.minor_ticks.first as f32;
                loop {
                    let angle = self.step * i;
                    let p1 = minor.get_point(angle);
                    let p2 = outer.get_point(angle);

                    let tick = Path::line(p1, p2);
                    frame.with_save(|frame| {
                        frame.stroke(&tick, stroke(width * 0.8, Color::WHITE));
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
                        frame.stroke(&tick, stroke(width * 1.5, Color::BLACK));
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
