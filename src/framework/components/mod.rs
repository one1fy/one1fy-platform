pub struct Style {
    pub color: Color,
}

impl Style {
    pub fn new(color: Color) -> Style {
        Style {
            color,
        }
    }
}

pub struct Color {
    pub color: u32,
}

impl Color {
    pub fn from_rgb(red: u8, blue: u8, green: u8) -> Color {
        let mut color: u32 = 0xffffffff;

        let mut r: u32 = red.into();
        r <<= 16;

        let mut b: u32 = blue.into();
        b <<= 8;

        color &= r;
        color &= b;
        color &= green as u32;

        Color {
            color,
        }
    }

    pub fn from_hex(hex: u32) -> Color {
        let mut alpha: u32 = 0xff000000;

        alpha = alpha | hex;

        Color {
            color: alpha,
        }
    }

    pub fn set_alpha(&mut self, alpha: u8) {
        let mut mask: u32 = alpha.into();
        mask <<= 24;
        self.color |= mask;
    }
}

pub struct BoxComponent {
    pub left: f32,
    pub top: f32,
    pub height: f32,
    pub width: f32,
    pub style: Style,
}

impl BoxComponent {
    pub fn new(
        left: f32,
        top: f32,
        height: f32,
        width: f32,
        style: Style,
    ) -> BoxComponent {
        BoxComponent {
            left,
            top,
            height,
            width,
            style,
        }
    }
}
