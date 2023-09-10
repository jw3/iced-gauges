use iced::widget::{canvas, container, toggler, Column, Row};
use iced::Event::Window;
use iced::{
    executor, window, Application, Command, Element, Length, Renderer, Subscription, Theme,
};
use iced::{Color, Settings};
use std::time::Duration;

use crate::Msg::Update;
use iced::time;
use iced_gauges::round::{Closing, Gauge};
use iced_gauges::tick::DefaultTick;
use iced_gauges::Tick;

fn main() -> iced::Result {
    Dashboard::run(Settings {
        antialiasing: true,
        window: window::Settings {
            size: (1280, 800),
            ..window::Settings::default()
        },
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
    dark_mode: bool,
}

#[derive(Debug)]
enum Msg {
    Update,
    ThemeChange(bool),
}

impl Application for Dashboard {
    type Executor = executor::Default;
    type Message = Msg;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let ticks = Box::new(DefaultTick {
            first: 0.0,
            major_step: 5.0,
            minor_step: 1.0,
            major_color: Color::BLACK,
            minor_color: Color::WHITE,
            major_length: 0.30,
            minor_length: 0.175,
            label: true,
            width: 1.5,
        });
        (
            Dashboard {
                gauge: vec![
                    Gauge::new(0.0, 85.0, 0.90, 0.30, Closing::Segment, ticks.clone()),
                    Gauge::new(0.0, 85.0, 0.90, 0.30, Closing::Segment, ticks.clone()),
                    Gauge::new(0.0, 85.0, 0.90, 0.30, Closing::Segment, ticks.clone()),
                    Gauge::new(0.0, 85.0, 0.35, 0.90, Closing::None, ticks.clone()),
                    Gauge::new(0.0, 85.0, 0.35, 0.40, Closing::Sector, ticks.clone()),
                    Gauge::new(0.0, 85.0, 0.35, 0.90, Closing::None, ticks.clone()),
                    Gauge::new(0.0, 85.0, 0.75, 0.0, Closing::None, ticks.clone()),
                    Gauge::new(0.0, 42.5, 1.0, 0.75, Closing::None, ticks.clone()),
                    Gauge::new(0.0, 85.0, 0.50, 0.30, Closing::Sector, ticks.clone()),
                ],
                state: State::Accel(0.0),
                dark_mode: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Dashboard demo for Round Gauge")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Update => match self.state {
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
            },
            Msg::ThemeChange(b) => {
                self.dark_mode = b;
                self.gauge.iter().for_each(|g| g.repaint());
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        // row
        //  col
        //   gauge 200x200
        //   gauge 200x200
        //  col
        //   gauge 500x500
        //   gauge 500x500
        //  col
        //   gauge 200x200
        //   gauge 200x200
        // row
        //  indicator leds?....

        let mut gauges = self.gauge.iter();
        let bar = Row::new().push(container(toggler(
            Some("Dark Mode".to_string()),
            self.dark_mode,
            Msg::ThemeChange,
        )));

        let top = Row::new()
            .push(canvas(gauges.next().unwrap()).width(200).height(200))
            .push(canvas(gauges.next().unwrap()).width(500).height(500))
            .push(canvas(gauges.next().unwrap()).width(500).height(500))
            .push(canvas(gauges.next().unwrap()).width(200).height(200));

        let bottom = Row::new()
            .push(canvas(gauges.next().unwrap()).width(200).height(200))
            .push(canvas(gauges.next().unwrap()).width(200).height(200))
            .push(canvas(gauges.next().unwrap()).width(200).height(200));

        let row = Column::new().push(bar).push(top).push(bottom);
        container(Column::new().push(row))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Self::Theme {
        if self.dark_mode {
            Theme::Dark
        } else {
            Theme::Light
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        use State::*;

        match self.state {
            Stop => Subscription::none(),
            Accel(_) | Decel(_) => time::every(Duration::from_millis(100)).map(|_| Update),
        }
    }
}
