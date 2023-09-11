use iced::widget::{canvas, container, Row};
use iced::Settings;
use iced::{executor, Application, Command, Element, Length, Renderer, Theme};

use iced_gauges::pin::Pins;
use iced_gauges::round::{Closing, Gauge};
use iced_gauges::style::Style;
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
        let ticks = MajorMinor::boxed(0.0, 25.0, 5.0, 0.30, true);
        let mut gauge = Gauge::new(0.0, 100.0, 0.30, 0.60, Closing::None, ticks, Style::Default);
        gauge.pin = Box::new(Pins::Large);

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
