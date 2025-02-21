use macroquad::color::Color;

pub struct Styles {
    pub colors: Colors,
}

impl Styles {
    pub fn new() -> Self {
        let colors = Colors {
            grey_dark: Color::from_hex(0x102e30),
            grey_mid: Color::from_hex(0x1b5d50),
            grey_light: Color::from_hex(0x1d6d60),

            white: Color::from_hex(0xf1f1f1),
            black: Color::from_hex(0x050505),

            red_light: Color::from_hex(0xf52d37),
            red_dark: Color::from_hex(0x771e16),
        };

        Self { colors }
    }
}

pub struct Colors {
    pub grey_dark: Color,
    pub grey_mid: Color,
    pub grey_light: Color,

    pub white: Color,
    pub black: Color,

    pub red_light: Color,
    pub red_dark: Color,
}
