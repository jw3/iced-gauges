use std::f32::consts::TAU;

use iced::widget::canvas::path::arc::Elliptical;
use iced::widget::canvas::path::Builder;
use iced::widget::canvas::{stroke, Cache, Cursor, Geometry, LineCap, Path, Program, Stroke};
use iced::{Color, Point, Rectangle, Theme, Vector};

use crate::needle::{Needle, Needles};
use crate::pin::{Pin, PinOrder, Pins};
use crate::style::Style;
use crate::util::frame;
use crate::Tick;

pub enum Closing {
    None,
    Segment,
    Sector,
}

pub type Radians = f32;

pub struct Gauge {
    /// Current unit value
    value: f32,
    needle_gfx: Cache,
    bg_gfx: Cache,
    border_gfx: Cache,
    ticks_gfx: Cache,
    pin_gfx: Cache,
    /// Radians of needle movement
    length: Radians,
    /// Radians of rotation
    rotate: Radians,
    /// Unit value minimum
    min: f32,
    /// Unit value maximum
    max: f32,
    /// Radians of movement per unit value
    step: Radians,
    /// Number of displayable unit steps at current resolution
    steps: usize,
    closing: Closing,
    pub ticks: Box<dyn Tick>,
    pub needle: Box<dyn Needle>,
    pub pin: Box<dyn Pin>,
    pub style: Style,
}

impl Gauge {
    // length and rotate are fractions of 1.0 which is a full circle
    // the value of these can exceed 1.0, in which case it loops
    pub fn new(
        min: f32,
        max: f32,
        length: f32,
        rotate: f32,
        closing: Closing,
        ticks: Box<dyn Tick>,
        style: Style,
    ) -> Self {
        // wait for builder impl
        let res = 1.0; // resolution: ie. visible values

        // derived
        let rotate = TAU * rotate;
        let length = TAU * length;
        let steps = (max - min) / res;
        let step = length / steps;

        Self {
            value: 0.0,
            needle_gfx: Default::default(),
            bg_gfx: Default::default(),
            border_gfx: Default::default(),
            ticks_gfx: Default::default(),
            pin_gfx: Default::default(),
            length,
            rotate,
            min,
            max,
            step,
            steps: steps as usize,
            closing,
            ticks,
            needle: Box::new(Needles::Diamond),
            pin: Box::new(Pins::Solid),
            style,
        }
    }

    #[must_use]
    pub fn with_needle(mut self, needle: Box<dyn Needle>) -> Self {
        self.needle = needle;
        self
    }

    #[must_use]
    pub fn with_pin(mut self, pin: Box<dyn Pin>) -> Self {
        self.pin = pin;
        self
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

    fn bg_path(&self, center: Point, radius: f32) -> Path {
        match self.closing {
            Closing::None => Path::circle(center, radius),
            Closing::Sector => {
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
            Closing::Segment => {
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
        }
    }

    fn stroke<'a>(&self, width: f32, color: Color) -> Stroke<'a> {
        Stroke {
            width,
            style: stroke::Style::Solid(color),
            line_cap: LineCap::Round,
            ..Stroke::default()
        }
    }
}

impl<M> Program<M> for Gauge {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let style = self.style.for_theme(theme);

        let bg = self.bg_gfx.draw(bounds.size(), |frame| {
            let frame_radius = frame::radius(frame);
            let border_radius = frame_radius - frame_radius / style.border_width_ratio;

            let background = self.bg_path(frame.center(), border_radius);
            frame.fill(&background, style.background_color);
        });

        let border = self.border_gfx.draw(bounds.size(), |frame| {
            let frame_radius = frame::radius(frame);
            let border_width = frame_radius / style.border_width_ratio;
            let border_inner_radius = frame_radius - border_width;

            frame.stroke(
                &self.bg_path(frame.center(), border_inner_radius),
                self.stroke(border_width, style.border_color),
            );
        });

        let needle = self.needle_gfx.draw(bounds.size(), |frame| {
            let center = frame.center();
            frame.with_save(|frame| {
                frame.translate(Vector::new(center.x, center.y));
                frame.rotate(self.rotate);
                frame.rotate(self.value * self.step);
                self.needle.draw(frame::radius(frame), self.value, frame);
            });
        });

        let ticks = self.ticks_gfx.draw(bounds.size(), |frame| {
            let center = frame.center();
            frame.with_save(|frame| {
                frame.translate(Vector::new(center.x, center.y));
                frame.rotate(self.rotate);
                self.ticks.draw(frame, &style, self.length, self.step);
            });
        });

        let pin = self.pin_gfx.draw(bounds.size(), |frame| {
            self.pin.draw(frame, style);
        });

        match style.pin_style {
            PinOrder::Over => vec![bg, ticks, border, needle, pin],
            PinOrder::Under => vec![bg, ticks, border, pin, needle],
        }
    }
}
