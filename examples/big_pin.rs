use iced::widget::{canvas, container, Row};
use iced::{Element, Length};
use iced::{Settings, Task};
use iced_gauges::needle::Needles;

use iced_gauges::pin::Pins;
use iced_gauges::round::{Closing, Gauge};
use iced_gauges::style::{Appearance, Style};
use iced_gauges::tick::MajorMinor;

fn main() -> iced::Result {
    iced::application("Big Pin Demo", Dashboard::update, Dashboard::view)
        .settings(Settings {
            antialiasing: true,
            ..Settings::default()
        })
        .run_with(Dashboard::new)
}

#[derive(Debug, Clone)]
enum Message {
    Next,
}

struct Dashboard {
    gauge: Gauge,
}

impl Dashboard {
    fn new() -> (Self, Task<Message>) {
        let style = Style::Custom(Appearance {
            pin_diameter_ratio: 1.0,
            pin_border_width_ratio: 0.10,
            ..Default::default()
        });

        let ticks = MajorMinor::boxed(0.0, 25.0, 5.0, 0.30);
        let mut gauge = Gauge::new(0.0, 100.0, 0.30, 0.60, Closing::None, ticks, style);
        gauge.pin = Box::new(Pins::Hollow);
        gauge.needle = Box::new(Needles::Triangle);
        (Dashboard { gauge }, Task::none())
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        Task::none()
    }
    fn view(&self) -> Element<Message> {
        container(Row::new().push(canvas(&self.gauge).width(500).height(500)))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
