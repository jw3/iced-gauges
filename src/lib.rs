//! Iced Gauges
pub use crate::util::Ellipse;

pub mod round;
pub mod style;
mod util;

#[derive(Copy, Clone)]
pub struct Ticks {
    pub first: f32,
    pub every: f32,
}
