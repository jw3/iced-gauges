use iced::widget::canvas::Frame;

pub(crate) fn radius(frame: &Frame) -> f32 {
    frame.width().min(frame.height()) / 2.0
}
