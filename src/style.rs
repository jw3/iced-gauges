use iced::{Color, Theme};

#[derive(Clone, Copy, Debug)]
pub struct Appearance {
    pub background_color: Color,

    /// Border thickness as a ratio of the frame radius
    pub border_width_ratio: f32,
    pub tick_border_width_ratio: f32,

    pub border_color: Color,

    pub tick_text_color: Color,

    pub major_tick_color: Color,
    pub minor_tick_color: Color,

    /// Ratio of radius
    pub major_tick_width_ratio: f32,

    /// Ratio of radius
    pub minor_tick_width_ratio: f32,

    /// Spacing of tick border from center as ratio of radius
    pub tick_border_inset_ratio: f32,

    pub pin_color: Color,
    pub pin_border_color: Color,
    /// Diameter of pin as ratio of radius
    pub pin_diameter_ratio: f32,
    /// Width of pin border as ratio of radius
    pub pin_border_width_ratio: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Style {
    #[default]
    Default,
    Custom(Appearance),
    Themed {
        light: Appearance,
        dark: Appearance,
    },
}

impl Style {
    pub fn for_theme(&self, theme: &Theme) -> &Appearance {
        match self {
            Style::Default => match theme {
                Theme::Light => &LIGHT_DEFAULT,
                _ => &DARK_DEFAULT,
            },
            Style::Custom(a) => a,
            Style::Themed { light, dark } => match theme {
                Theme::Light => light,
                _ => dark,
            },
        }
    }
}

impl Default for Appearance {
    fn default() -> Self {
        LIGHT_DEFAULT
    }
}

const LIGHT_DEFAULT: Appearance = Appearance {
    background_color: Color::from_rgb(18.0 / 255.0, 146.0 / 255.0, 216.0 / 255.0),
    border_width_ratio: 50.0,
    tick_border_width_ratio: 1.0,
    tick_border_inset_ratio: 0.8,
    border_color: Color::BLACK,
    tick_text_color: Color::BLACK,
    major_tick_color: Color::BLACK,
    minor_tick_color: Color::WHITE,
    major_tick_width_ratio: 1.0,
    minor_tick_width_ratio: 0.8,

    pin_color: Color::WHITE,
    pin_border_color: Color::BLACK,
    pin_diameter_ratio: 0.5,
    pin_border_width_ratio: 0.0,
};

const DARK_DEFAULT: Appearance = Appearance {
    background_color: Color::from_rgb(48.0 / 255.0, 71.0 / 255.0, 94.0 / 255.0),
    border_width_ratio: 50.0,
    tick_border_width_ratio: 1.0,
    tick_border_inset_ratio: 0.8,
    border_color: Color::from_rgba(246.0 / 255.0, 88.0 / 255.0, 7.0 / 255.0, 1.0),
    tick_text_color: Color::BLACK,
    major_tick_color: Color::BLACK,
    minor_tick_color: Color::from_rgba(246.0 / 255.0, 88.0 / 255.0, 7.0 / 255.0, 1.0),
    major_tick_width_ratio: 1.0,
    minor_tick_width_ratio: 0.8,

    pin_color: Color::WHITE,
    pin_border_color: Color::BLACK,
    pin_diameter_ratio: 0.5,
    pin_border_width_ratio: 0.0,
};
