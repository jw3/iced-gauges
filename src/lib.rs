//! Iced Gauges
pub use ellipse::Ellipse;

pub mod configurable_round;
mod ellipse;
pub mod full_round;
pub mod half_round;
pub mod half_round_rotated;
pub mod pct30_round_rotated;
pub mod round;

pub struct Ticks {
    first: usize,
    every: usize,
}
