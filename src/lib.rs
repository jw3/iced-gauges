//! Iced Gauges
pub use ellipse::Ellipse;

mod ellipse;
pub mod round;

#[derive(Copy, Clone)]
pub struct Ticks {
    pub first: f32,
    pub every: f32,
}
