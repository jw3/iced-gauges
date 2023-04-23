use iced::widget::{canvas, container};
use iced::Settings;
use iced::{executor, Application, Command, Element, Length, Renderer, Subscription, Theme};
use std::time::Duration;

use iced::time;
use iced_gauges::half_round_rotated::Gauge;

fn main() -> iced::Result {
    Dashboard::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

enum State {
    Stop,
    Accel(f32),
    Decel(f32),
}

struct Dashboard {
    gauge: Gauge,
    state: State,
}

impl Application for Dashboard {
    type Executor = executor::Default;
    type Message = ();
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Dashboard {
                gauge: Gauge::new(),
                state: State::Accel(0.0),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Dashboard demo for Round Gauge")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        match self.state {
            State::Accel(v) => {
                if v < 120.0 {
                    let v = v + 1.0;
                    self.state = State::Accel(v);
                    self.gauge.update_value(v);
                } else {
                    self.state = State::Decel(120.0);
                }
            }
            State::Decel(v) => {
                if v <= 0.0 {
                    self.state = State::Stop;
                    self.gauge.update_value(0.0);
                } else {
                    let v = v - 1.0;
                    self.state = State::Decel(v);
                    self.gauge.update_value(v);
                }
            }
            State::Stop => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        let canvas = canvas(&self.gauge).width(Length::Fill).height(Length::Fill);
        container(canvas)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        use State::*;

        match self.state {
            Stop => Subscription::none(),
            Accel(_) | Decel(_) => time::every(Duration::from_millis(100)).map(|_| ()),
        }
    }
}
