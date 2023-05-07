use iced::Color;

#[derive(Copy, Clone)]
pub struct Tick {
    pub first: f32,
    pub every: f32,
    pub color: Color,
    /// ratio of radius 0.0 -- 1.0
    pub length: f32,
    pub label: bool,
    pub width: f32,
    pub skip: Option<usize>,
}
