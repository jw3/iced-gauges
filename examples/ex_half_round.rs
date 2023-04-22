use iced::widget::{canvas, container};
use iced::{Element, Length, Sandbox, Settings};
use iced_gauges::half_round::Gauge;

fn main() -> iced::Result {
    Dashboard::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

struct Dashboard {
    gauge: Gauge,
}

impl Sandbox for Dashboard {
    type Message = ();

    fn new() -> Self {
        Dashboard {
            gauge: Gauge::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Gauge - Half Round")
    }

    fn update(&mut self, _message: Self::Message) {
        todo!()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let canvas = canvas(&self.gauge).width(Length::Fill).height(Length::Fill);
        container(canvas)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }
}
