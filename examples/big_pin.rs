use iced::widget::{canvas, container, Row};
use iced::{executor, Application, Command, Element, Length, Renderer, Theme};
use iced::{Color, Settings};
use iced_gauges::pin::Pins;
use iced_gauges::round::{Closing, Gauge};
use iced_gauges::tick::DefaultTick;
use iced_gauges::Tick;

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
        let ticks = Box::new(DefaultTick {
            first: 0.0,
            major_step: 25.0,
            minor_step: 5.0,
            major_color: Color::BLACK,
            minor_color: Color::WHITE,
            major_length: 0.30,
            minor_length: 0.175,
            label: true,
            width: 1.5,
        });
        let mut gauge = Gauge::new(0.0, 100.0, 0.30, 0.60, Closing::None, ticks);
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
