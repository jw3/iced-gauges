use iced::widget::canvas::Frame;
use iced::Color;

#[derive(Copy, Clone, Debug)]
pub struct Tick {
    /// radian position of the first tick
    pub first: f32,
    /// the number of units of measure (uom) between ticks
    pub step: f32,
    /// the number of steps to represent
    pub steps: Option<usize>,
    /// length of stroke, as ratio of radius 0.0 -- 1.0
    pub length: f32,
    /// width of stroke
    pub width: f32,
    pub label: bool,
    pub skip: Option<usize>,
    pub color: Color,
}
