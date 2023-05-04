use crate::{Ellipse, Ticks};
use iced::theme::palette::Extended;
use iced::widget::canvas::path::arc::Elliptical;
use iced::widget::canvas::path::Builder;
use iced::widget::canvas::{stroke, Cache, Cursor, Geometry, LineCap, Path, Program, Stroke};
use iced::{Color, Point, Rectangle, Theme, Vector};
use std::f32::consts::TAU;

pub enum Closure {
    None,
    Segment,
    Sector,
}

pub struct Gauge {
    value: f32,
    needle_gfx: Cache,
    bg_gfx: Cache,
    border_gfx: Cache,
    ticks_gfx: Cache,
    pin_gfx: Cache,
    length: f32,
    rotate: f32,
    major_ticks: Ticks,
    minor_ticks: Ticks,
    min: f32,
    max: f32,
    step: f32,
    closure: Closure,
}

impl Gauge {
    // length and rotate are fractions of 1.0 which is a full circle
    // the value of these can exceed 1.0, in which case it loops
    pub fn new(
        min: f32,
        max: f32,
        length: f32,
        rotate: f32,
        closure: Closure,
        major_ticks: Ticks,
        minor_ticks: Ticks,
    ) -> Self {
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
            border_gfx: Default::default(),
            ticks_gfx: Default::default(),
            pin_gfx: Default::default(),
            length,
            rotate,
            major_ticks,
            minor_ticks,
            min,
            max,
            step,
            closure,
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

    pub fn repaint(&self) {
        self.needle_gfx.clear();
        self.bg_gfx.clear();
        self.ticks_gfx.clear();
        self.pin_gfx.clear();
        self.border_gfx.clear();
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    pub background: Color,
    pub border: Color,
}

impl Style {
    pub const LIGHT: Style = Style {
        background: Color::from_rgb(18.0 / 255.0, 147.0 / 255.0, 216.0 / 255.0),
        border: Color::BLACK,
    };

    pub const DARK: Style = Style {
        background: Color::from_rgb(48.0 / 255.0, 71.0 as f32 / 255.0, 94.0 as f32 / 255.0),
        border: Color {
            r: 246.0 / 255.0,
            g: 88.0 / 255.0,
            b: 7.0 / 255.0,
            a: 1.0,
        },
    };
}

impl From<&Theme> for Style {
    fn from(value: &Theme) -> Self {
        match value {
            Theme::Light => Style::LIGHT,
            Theme::Dark => Style::DARK,
            Theme::Custom(_) => Style::from(value.extended_palette()),
        }
    }
}

impl From<&Extended> for Style {
    fn from(x: &Extended) -> Self {
        Self {
            background: x.background.base.color,
            border: x.secondary.weak.color,
        }
    }
}

impl<T> Program<T> for Gauge {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let bg = self.bg_gfx.draw(bounds.size(), |frame| {
            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 2.5;

            let background = match self.closure {
                Closure::None => Path::circle(center, radius),
                Closure::Sector => {
                    let mut builder = Builder::new();
                    builder.ellipse(Elliptical {
                        center,
                        radii: Vector::new(radius, radius),
                        rotation: self.rotate,
                        start_angle: 0.0,
                        end_angle: self.length,
                    });
                    builder.line_to(center);
                    builder.close();
                    builder.build()
                }
                Closure::Segment => {
                    let mut builder = Builder::new();
                    builder.ellipse(Elliptical {
                        center,
                        radii: Vector::new(radius, radius),
                        rotation: self.rotate,
                        start_angle: 0.0,
                        end_angle: self.length,
                    });
                    builder.close();
                    builder.build()
                }
            };

            let style = Style::from(theme);
            frame.fill(&background, style.background);
        });

        let border = self.border_gfx.draw(bounds.size(), |frame| {
            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 2.5;
            let width = radius / 50.0;

            let stroke = |color: Color| -> Stroke {
                Stroke {
                    width,
                    style: stroke::Style::Solid(color),
                    line_cap: LineCap::Round,
                    ..Stroke::default()
                }
            };

            let background = match self.closure {
                Closure::None => Path::circle(center, radius),
                Closure::Sector => {
                    let mut builder = Builder::new();
                    builder.ellipse(Elliptical {
                        center,
                        radii: Vector::new(radius, radius),
                        rotation: self.rotate,
                        start_angle: 0.0,
                        end_angle: self.length,
                    });
                    builder.line_to(center);
                    builder.close();
                    builder.build()
                }
                Closure::Segment => {
                    let mut builder = Builder::new();
                    builder.ellipse(Elliptical {
                        center,
                        radii: Vector::new(radius, radius),
                        rotation: self.rotate,
                        start_angle: 0.0,
                        end_angle: self.length,
                    });
                    builder.close();
                    builder.build()
                }
            };

            let style = Style::from(theme);
            frame.stroke(&background, stroke(style.border));
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

                let mut i = self.minor_ticks.first;
                loop {
                    let angle = self.step * i;
                    let p1 = minor.get_point(angle);
                    let p2 = outer.get_point(angle);

                    let tick = Path::line(p1, p2);
                    frame.with_save(|frame| {
                        frame.stroke(&tick, stroke(width * 0.8, Color::WHITE));
                        frame.translate(Vector::new(p1.x, p1.y));
                    });

                    i += self.minor_ticks.every;
                    if i * self.step >= self.length {
                        break;
                    }
                }

                let mut i = self.major_ticks.first;
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

                    if i * self.step >= self.length {
                        break;
                    }
                    i += self.major_ticks.every;
                }
            });
        });

        let pin = self.pin_gfx.draw(bounds.size(), |frame| {
            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 2.5;
            let dot = Path::circle(center, radius / 25.0);
            frame.fill(&dot, Color::BLACK);
        });

        vec![bg, ticks, border, pin, needle]
    }
}
