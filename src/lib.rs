//! Iced Gauges
pub use ellipse::Ellipse;

mod ellipse;
pub mod round;

pub struct Ticks {
    first: usize,
    every: usize,
}
