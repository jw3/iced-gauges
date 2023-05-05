//! Iced Gauges
pub use crate::util::Ellipse;
use iced::Color;

pub mod round;
mod util;

#[derive(Copy, Clone)]
pub struct Ticks {
    pub first: f32,
    pub every: f32,
    pub color: Color,
    /// ratio of radius 0.0 -- 1.0
    pub length: f32,
    pub label: bool,
}
