use iced::widget::{canvas, container, toggler, Column, Row};
use iced::{
    executor, window, Application, Command, Element, Length, Renderer, Subscription, Theme,
};
use iced::{Color, Settings};
use std::time::Duration;

use crate::Msg::Update;
use iced::time;
use iced_gauges::needle::Needles;
use iced_gauges::pin::Pins;
use iced_gauges::round::{Closing, Gauge};
use iced_gauges::style::{Appearance, Style, DARK_DEFAULT, LIGHT_DEFAULT};
use iced_gauges::tick::MajorMinor;

fn main() -> iced::Result {
    Dashboard::run(Settings {
        antialiasing: true,
        window: window::Settings {
            size: (1000, 700),
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
        let ticks = MajorMinor::boxed(0.0, 5.0, 1.0, 0.30);
        let small_ticks = MajorMinor::boxed(0.0, 25.0, 5.0, 0.30);
        let style = Style::Themed {
            light: Appearance {
                pin_border_width_ratio: 0.1,
                pin_diameter_ratio: 0.5,
                pin_color: Color::from_rgb(0.5, 0.5, 0.5),
                ..LIGHT_DEFAULT
            },
            dark: Appearance {
                pin_border_width_ratio: 0.1,
                pin_diameter_ratio: 0.5,
                tick_text_color: Color::WHITE,
                ..DARK_DEFAULT
            },
        };
        (
            Dashboard {
                gauge: vec![
                    Gauge::new(
                        0.0,
                        85.0,
                        0.90,
                        0.30,
                        Closing::Segment,
                        ticks.clone(),
                        style,
                    )
                    .with_needle(Box::new(Needles::Arrow))
                    .with_pin(Box::new(Pins::Hollow)),
                    Gauge::new(
                        0.0,
                        125.0,
                        0.35,
                        0.60,
                        Closing::None,
                        small_ticks.clone(),
                        style,
                    ),
                    Gauge::new(
                        0.0,
                        100.0,
                        0.90,
                        0.30,
                        Closing::Segment,
                        small_ticks.clone(),
                        style,
                    ),
                    Gauge::new(
                        0.0,
                        100.0,
                        0.35,
                        0.40,
                        Closing::Sector,
                        small_ticks.clone(),
                        style,
                    )
                    .with_needle(Box::new(Needles::Arrow)),
                    Gauge::new(
                        0.0,
                        100.0,
                        0.35,
                        0.90,
                        Closing::None,
                        small_ticks.clone(),
                        style,
                    ),
                    Gauge::new(
                        0.0,
                        100.0,
                        0.75,
                        0.0,
                        Closing::None,
                        small_ticks.clone(),
                        style,
                    ),
                    Gauge::new(
                        0.0,
                        85.0,
                        0.90,
                        0.30,
                        Closing::Segment,
                        ticks.clone(),
                        Style::Themed {
                            light: Appearance {
                                tick_labels: false,
                                pin_diameter_ratio: 0.8,
                                ..Appearance::default()
                            },
                            dark: Appearance {
                                tick_labels: false,
                                pin_diameter_ratio: 0.8,
                                ..Appearance::default()
                            },
                        },
                    )
                    .with_needle(Box::new(Needles::Triangle))
                    .with_pin(Box::new(Pins::Solid)),
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
            .push(canvas(gauges.next().unwrap()).width(500).height(500))
            .push(canvas(gauges.next().unwrap()).width(500).height(500));

        let bottom = Row::new()
            .push(canvas(gauges.next().unwrap()).width(200).height(200))
            .push(canvas(gauges.next().unwrap()).width(200).height(200))
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
