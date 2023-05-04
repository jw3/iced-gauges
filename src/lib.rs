//! Iced Gauges
pub use crate::util::Ellipse;

pub mod round;
mod util;

#[derive(Copy, Clone)]
pub struct Ticks {
    pub first: f32,
    pub every: f32,
}
