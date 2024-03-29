use iced::mouse::Cursor;
use iced::widget::canvas::{stroke, Cache, Geometry, LineCap, Path, Stroke};
use iced::widget::{canvas, container};
use iced::{
    executor, Application, Color, Command, Element, Length, Point, Rectangle, Renderer, Settings,
    Subscription, Theme, Vector,
};
use iced_gauges::Ellipse;
use std::f32::consts::PI;

fn main() -> iced::Result {
    Clock::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

struct Clock {
    v: time::OffsetDateTime,
    hands: Cache,
    bg: Cache,
    frame: Cache,
}

#[derive(Debug, Clone, Copy)]
enum Msg {
    Tick(time::OffsetDateTime),
}

impl Application for Clock {
    type Executor = executor::Default;
    type Message = Msg;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Clock {
                v: time::OffsetDateTime::now_local()
                    .unwrap_or_else(|_| time::OffsetDateTime::now_utc()),
                hands: Default::default(),
                bg: Default::default(),
                frame: Default::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Clock -- Round")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Msg::Tick(local_time) => {
                let now = local_time;
                if now != self.v {
                    self.v = now;
                    self.hands.clear();
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        let canvas = canvas(self).width(Length::Fill).height(Length::Fill);
        container(canvas)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }

    fn subscription(&self) -> Subscription<Msg> {
        iced::time::every(std::time::Duration::from_millis(500)).map(|_| {
            Msg::Tick(
                time::OffsetDateTime::now_local()
                    .unwrap_or_else(|_| time::OffsetDateTime::now_utc()),
            )
        })
    }
}

impl canvas::Program<Msg> for Clock {
    type State = ();

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Vec<Geometry> {
        let bg = self.bg.draw(renderer, bounds.size(), |frame| {
            println!("drawing bg");
            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 2.5;

            let background = Path::circle(center, radius);
            frame.fill(&background, Color::from_rgb8(0x12, 0x93, 0xD8));
        });

        let g = self.hands.draw(renderer, bounds.size(), |frame| {
            println!("drawing hands");

            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 2.5;

            let width = radius / 100.0;
            let thin_stroke = |color: Color| -> Stroke {
                Stroke {
                    width,
                    style: stroke::Style::Solid(color),
                    line_cap: LineCap::Round,
                    ..Stroke::default()
                }
            };

            let wide_stroke = |color: Color| -> Stroke {
                Stroke {
                    width: width * 3.0,
                    style: stroke::Style::Solid(color),
                    line_cap: LineCap::Round,
                    ..Stroke::default()
                }
            };

            let length = PI * 2.0;
            let step_12 = length / 12.0;
            let step_60 = length / 60.0;

            let short_hand = Path::line(Point::ORIGIN, Point::new(0.0, -0.5 * radius));
            let long_hand = Path::line(Point::ORIGIN, Point::new(0.0, -0.8 * radius));

            frame.translate(Vector::new(center.x, center.y));

            frame.with_save(|frame| {
                frame.rotate(self.v.hour() as f32 * step_12);
                frame.stroke(&short_hand, wide_stroke(Color::WHITE));

                frame.translate(Vector::new(0.0, -0.5 * radius));
                frame.fill_text(format!("{}", self.v.hour()));
            });

            frame.with_save(|frame| {
                frame.rotate(self.v.minute() as f32 * step_60);
                frame.stroke(&long_hand, wide_stroke(Color::WHITE));

                frame.translate(Vector::new(0.0, -0.8 * radius));
                frame.fill_text(format!("{}", self.v.minute()));
            });

            frame.with_save(|frame| {
                frame.rotate(self.v.second() as f32 * step_60);
                frame.stroke(&long_hand, thin_stroke(Color::WHITE));

                frame.translate(Vector::new(0.0, -0.8 * radius));
                frame.fill_text(format!("{}", self.v.second()));
            });
        });

        let frame = self.frame.draw(renderer, bounds.size(), |frame| {
            println!("drawing frame");

            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 2.5;
            let width = radius / 100.0;

            let thin_stroke = |color: Color| -> Stroke {
                Stroke {
                    width,
                    style: stroke::Style::Solid(color),
                    line_cap: LineCap::Round,
                    ..Stroke::default()
                }
            };

            frame.with_save(|frame| {
                frame.translate(Vector::new(center.x, center.y));

                let length = PI * 2.0;
                let step_12 = length / 12.0;

                frame.rotate(PI * 1.5);

                let outer = Ellipse {
                    center: Point::ORIGIN,
                    major_curvature: 1.0 / radius,
                    minor_radius: radius,
                    angle: 0.0,
                };

                let radius2 = radius - 25.0;
                let inner = Ellipse {
                    center: Point::ORIGIN,
                    major_curvature: 1.0 / radius2,
                    minor_radius: radius2,
                    angle: 0.0,
                };

                // hour ticks
                for a in 0..12 {
                    let angle = step_12 * a as f32;
                    let p1 = inner.get_point(angle);
                    let p2 = outer.get_point(angle);

                    let tick = Path::line(p1, p2);
                    frame.with_save(|frame| {
                        frame.stroke(&tick, thin_stroke(Color::BLACK));

                        frame.translate(Vector::new(p1.x, p1.y));
                        frame.fill_text(format!("{a}"));
                    });
                }
            });

            let width = radius / 100.0;
            let thin_stroke = |color: Color| -> Stroke {
                Stroke {
                    width,
                    style: stroke::Style::Solid(color),
                    line_cap: LineCap::Round,
                    ..Stroke::default()
                }
            };

            let dot = Path::circle(center, radius / 25.0);
            frame.fill(&dot, Color::BLACK);

            let background = Path::circle(center, radius);
            frame.stroke(&background, thin_stroke(Color::BLACK));
        });

        vec![bg, g, frame]
    }
}
