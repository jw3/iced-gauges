use iced::widget::canvas::{stroke, Cache, Cursor, Geometry, LineCap, Path, Stroke};
use iced::widget::{canvas, container};
use iced::{
    executor, Application, Color, Command, Element, Length, Point, Rectangle, Renderer, Settings,
    Subscription, Theme, Vector,
};
use iced_gauges::Ellipse;
use std::f32::consts::{FRAC_PI_2, PI};

fn main() -> iced::Result {
    Compass::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

struct Compass {
    bg: Cache,
    frame: Cache,
}

#[derive(Debug, Clone, Copy)]
enum Msg {
    Tick(time::OffsetDateTime),
}

impl Application for Compass {
    type Executor = executor::Default;
    type Message = Msg;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Compass {
                bg: Default::default(),
                frame: Default::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Compass -- Round")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
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
}

impl canvas::Program<Msg> for Compass {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let bg = self.bg.draw(bounds.size(), |frame| {
            println!("drawing bg");
            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 2.5;

            let background = Path::circle(center, radius);
            frame.fill(&background, Color::from_rgb8(0x12, 0x93, 0xD8));
        });

        let frame = self.frame.draw(bounds.size(), |frame| {
            println!("drawing frame");

            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 2.5;
            let width = 1.0;

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
                let step_36 = length / 36.0;
                let step_62 = length / 62.0;

                let outer = Ellipse {
                    center: Point::ORIGIN,
                    major_curvature: 1.0 / radius,
                    minor_radius: radius,
                    angle: 0.0,
                };

                let radius2 = radius + 20.0;
                let inner = Ellipse {
                    center: Point::ORIGIN,
                    major_curvature: 1.0 / radius2,
                    minor_radius: radius2,
                    angle: 0.0,
                };

                let radius3 = radius - 30.0;
                let outer2 = Ellipse {
                    center: Point::ORIGIN,
                    major_curvature: 1.0 / radius3,
                    minor_radius: radius3,
                    angle: 0.0,
                };

                for a in 0..62 {
                    let radian = step_62 * a as f32;

                    let p1 = inner.get_point(radian);
                    let p2 = outer.get_point(radian);

                    let tick = Path::line(p1, p2);
                    frame.with_save(|frame| {
                        frame.stroke(&tick, thin_stroke(Color::BLACK));

                        frame.with_save(|frame| {
                            frame.translate(Vector::new(p1.x, p1.y));
                            frame.fill_text(format!("{radian:.1}"));
                        });
                    });
                }

                for a in 0..36 {
                    let radian = step_36 * a as f32;

                    let p4 = outer.get_point(radian);
                    let p3 = outer2.get_point(radian);
                    let tick2 = Path::line(p3, p4);

                    frame.with_save(|frame| {
                        frame.stroke(&tick2, thin_stroke(Color::BLACK));
                        frame.translate(Vector::new(p3.x, p3.y));
                        let deg = radian.to_degrees();
                        frame.fill_text(format!("{deg:.1}"));
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

        vec![bg, frame]
    }
}
