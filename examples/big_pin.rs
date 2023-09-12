use iced::widget::{canvas, container, Row};
use iced::Settings;
use iced::{executor, Application, Command, Element, Length, Renderer, Theme};
use iced_gauges::needle::Needles;

use iced_gauges::pin::Pins;
use iced_gauges::round::{Closing, Gauge};
use iced_gauges::style::{Appearance, Style};
use iced_gauges::tick::MajorMinor;

struct Dashboard {
    gauge: Gauge,
}

fn main() -> iced::Result {
    Dashboard::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

impl Application for Dashboard {
    type Executor = executor::Default;
    type Message = ();
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let style = Style::Custom(Appearance {
            pin_diameter_ratio: 1.0,
            pin_border_width_ratio: 0.10,
            ..Default::default()
        });

        let ticks = MajorMinor::boxed(0.0, 25.0, 5.0, 0.30);
        let mut gauge = Gauge::new(0.0, 100.0, 0.30, 0.60, Closing::None, ticks, style);
        gauge.pin = Box::new(Pins::Hollow);
        gauge.needle = Box::new(Needles::Triangle);

        (Dashboard { gauge }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Dashboard demo for Round Gauge")
    }

    fn update(&mut self, _: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        container(Row::new().push(canvas(&self.gauge).width(500).height(500)))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
