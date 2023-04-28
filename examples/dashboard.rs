use iced::widget::{canvas, container, Column, Row};
use iced::Settings;
use iced::{executor, Application, Command, Element, Length, Renderer, Subscription, Theme};
use std::time::Duration;

use iced::time;
use iced_gauges::round::{Closure, Gauge};
use iced_gauges::Ticks;

fn main() -> iced::Result {
    Dashboard::run(Settings {
        antialiasing: false,
        ..Settings::default()
    })
}

enum State {
    Stop,
    Accel(f32),
    Decel(f32),
}

struct Dashboard {
    gauge: Vec<Gauge>,
    state: State,
}

impl Application for Dashboard {
    type Executor = executor::Default;
    type Message = ();
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let maj = Ticks {
            first: 0.0,
            every: 5.0,
        };
        let min = Ticks {
            first: 0.0,
            every: 1.0,
        };
        (
            Dashboard {
                gauge: vec![
                    Gauge::new(0.0, 85.0, 0.90, 0.30, Closure::Segment, maj, min),
                    Gauge::new(0.0, 85.0, 0.90, 0.30, Closure::Sector, maj, min),
                    Gauge::new(0.0, 85.0, 0.35, 0.40, Closure::Segment, maj, min),
                    Gauge::new(0.0, 85.0, 0.35, 0.90, Closure::Sector, maj, min),
                    Gauge::new(0.0, 85.0, 0.35, 0.40, Closure::Sector, maj, min),
                    Gauge::new(0.0, 85.0, 0.35, 0.90, Closure::Segment, maj, min),
                    Gauge::new(0.0, 85.0, 0.75, 0.0, Closure::Segment, maj, min),
                    Gauge::new(
                        0.0,
                        42.5,
                        1.0,
                        0.75,
                        Closure::Segment,
                        Ticks {
                            first: 0.0,
                            every: 2.5,
                        },
                        min,
                    ),
                    Gauge::new(0.0, 85.0, 0.50, 0.30, Closure::Sector, maj, min),
                ],
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
                if v < 85.0 {
                    let v = v + 1.0;
                    self.state = State::Accel(v);
                    self.gauge.iter_mut().for_each(|g| g.update_value(v));
                } else {
                    self.state = State::Decel(85.0);
                }
            }
            State::Decel(v) => {
                if v <= 0.0 {
                    self.state = State::Stop;
                    self.gauge.iter_mut().for_each(|g| g.update_value(0.0));
                } else {
                    let v = v - 1.0;
                    self.state = State::Decel(v);
                    self.gauge.iter_mut().for_each(|g| g.update_value(v));
                }
            }
            State::Stop => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        let mut body = Column::new();
        let mut gauges = self.gauge.iter();
        for _ in 0..3 {
            let mut row = Row::new();
            row = row.push(canvas(gauges.next().unwrap()).width(300).height(300));
            row = row.push(canvas(gauges.next().unwrap()).width(300).height(300));
            row = row.push(canvas(gauges.next().unwrap()).width(300).height(300));
            body = body.push(row);
        }
        container(body)
            .width(Length::Fill)
            .height(Length::Fill)
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
