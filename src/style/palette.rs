use iced::theme::palette::Extended;
use iced::{Color, Theme};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Palette {
    pub background: Color,
}

impl Palette {
    pub fn new(theme: &Theme) -> Self {
        match theme {
            Theme::Light => Palette::LIGHT,
            Theme::Dark => Palette::DARK,
            Theme::Custom(_) => Self::from_extended(theme.extended_palette()),
        }
    }

    fn from_extended(x: &Extended) -> Self {
        Self {
            background: x.background.base.color,
        }
    }

    pub const LIGHT: Self = Self {
        background: Color::from_rgb(18.0 / 255.0, 147.0 / 255.0, 216.0 / 255.0),
    };

    pub const DARK: Self = Self {
        background: Color::from_rgb(48.0 / 255.0, 71.0 as f32 / 255.0, 94.0 as f32 / 255.0),
    };
}
