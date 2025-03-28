use neutuino::ansi;

pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Default,
    Rgb(u8, u8, u8),
}

impl Color {
    pub fn as_string_fg(&self) -> String {
        let string = match self {
            Color::Black => ansi::COLOR_BLACK_FG,
            Color::Red => ansi::COLOR_RED_FG,
            Color::Green => ansi::COLOR_GREEN_FG,
            Color::Yellow => ansi::COLOR_YELLOW_FG,
            Color::Blue => ansi::COLOR_BLUE_FG,
            Color::Magenta => ansi::COLOR_MAGENTA_FG,
            Color::Cyan => ansi::COLOR_CYAN_FG,
            Color::White => ansi::COLOR_WHITE_FG,
            Color::Default => ansi::COLOR_DEFAULT_FG,
            Color::Rgb(r, g, b) => &ansi::rgb_color_code_fg(*r, *g, *b),
        };

        string.to_string()
    }

    pub fn as_string_bg(&self) -> String {
        let string = match self {
            Color::Black => ansi::COLOR_BLACK_BG,
            Color::Red => ansi::COLOR_RED_BG,
            Color::Green => ansi::COLOR_GREEN_BG,
            Color::Yellow => ansi::COLOR_YELLOW_BG,
            Color::Blue => ansi::COLOR_BLUE_BG,
            Color::Magenta => ansi::COLOR_MAGENTA_BG,
            Color::Cyan => ansi::COLOR_CYAN_BG,
            Color::White => ansi::COLOR_WHITE_BG,
            Color::Default => ansi::COLOR_DEFAULT_BG,
            Color::Rgb(r, g, b) => &ansi::rgb_color_code_bg(*r, *g, *b),
        };

        string.to_string()
    }
}

pub struct Style {
    pub fg_color: Color,
    pub bg_color: Color,
    pub bold: bool,
    pub dim: bool,
    pub italic: bool,
    pub underline: bool,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            fg_color: Color::Default,
            bg_color: Color::Default,
            bold: false,
            dim: false,
            italic: false,
            underline: false,
        }
    }
}

impl Style {
    pub fn apply(self) -> String {
        let mut style = String::new();

        style.push_str(self.fg_color.as_string_fg().as_str());
        style.push_str(self.bg_color.as_string_bg().as_str());

        if self.bold {
            style.push_str(ansi::STYLE_BOLD);
        }

        if self.dim {
            style.push_str(ansi::STYLE_DIM);
        }

        if self.italic {
            style.push_str(ansi::STYLE_ITALIC);
        }

        if self.underline {
            style.push_str(ansi::STYLE_UNDERLINE);
        }

        style
    }
}
