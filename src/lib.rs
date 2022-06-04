use rs3a::Color;

pub mod default_colors{
    pub const BLACK: &str =          "black";
    pub const BLUE: &str =           "navy";
    pub const GREEN: &str =          "green";
    pub const CYAN: &str =           "teal";
    pub const RED: &str =            "maroon";
    pub const MAGENTA: &str =        "purple";
    pub const YELLOW: &str =         "olive";
    pub const WHITE: &str =          "silver";
    pub const GRAY: &str =           "gray";
    pub const BRIGHT_BLUE: &str =    "blue";
    pub const BRIGHT_GREEN: &str =   "lime";
    pub const BRIGHT_CYAN: &str =    "aqua";
    pub const BRIGHT_RED: &str =     "red";
    pub const BRIGHT_MAGENTA: &str = "fuchsia";
    pub const BRIGHT_YELLOW: &str =  "yellow";
    pub const BRIGHT_WHITE: &str =   "white";
}

#[derive(Debug, Clone)]
pub struct Palette<'a> {
    pub fg: BasePalette<'a>,
    pub bg: BasePalette<'a>,
}

impl<'a> Palette<'a> {
    pub fn new() -> Self {
        Self{
            fg: BasePalette::new(),
            bg: BasePalette::new(),
        }
    }
    pub fn render(&self, prefix: &str) -> String{
        let mut ret = String::new();
        ret = format!("{}{}", ret, self.fg.render(format!("{}fg-", prefix).as_str(), ""));
        ret = format!("{}{}", ret, self.bg.render(format!("{}bg-", prefix).as_str(), "background-"));
        ret
    }
}

#[derive(Debug, Clone)]
pub struct BasePalette<'a> {
    black: Option<&'a str>,
    blue: Option<&'a str>,
    green: Option<&'a str>,
    cyan: Option<&'a str>,
    red: Option<&'a str>,
    magenta: Option<&'a str>,
    yellow: Option<&'a str>,
    white: Option<&'a str>,
    gray: Option<&'a str>,
    bright_blue: Option<&'a str>,
    bright_green: Option<&'a str>,
    bright_cyan: Option<&'a str>,
    bright_red: Option<&'a str>,
    bright_magenta: Option<&'a str>,
    bright_yellow: Option<&'a str>,
    bright_white: Option<&'a str>,
}

impl<'a> BasePalette<'a> {
    pub fn new() -> Self {
        BasePalette{
            black:          None,
            blue:           None,
            green:          None,
            cyan:           None,
            red:            None,
            magenta:        None,
            yellow:         None,
            white:          None,
            gray:           None,
            bright_blue:    None,
            bright_green:   None,
            bright_cyan:    None,
            bright_red:     None,
            bright_magenta: None,
            bright_yellow:  None,
            bright_white:   None,
        }
    }
    pub fn set(&mut self, color: Color, code: &'a str) {
        match color{
            Color::BLACK => {self.black = Some(code)}
            Color::BLUE => {self.blue = Some(code)}
            Color::GREEN => {self.green = Some(code)}
            Color::CYAN => {self.cyan = Some(code)}
            Color::RED => {self.red = Some(code)}
            Color::MAGENTA => {self.magenta = Some(code)}
            Color::YELLOW => {self.yellow = Some(code)}
            Color::WHITE => {self.white = Some(code)}
            Color::GRAY => {self.gray = Some(code)}
            Color::BRIGHT_BLUE => {self.bright_blue = Some(code)}
            Color::BRIGHT_GREEN => {self.bright_green = Some(code)}
            Color::BRIGHT_CYAN => {self.bright_cyan = Some(code)}
            Color::BRIGHT_RED => {self.bright_red = Some(code)}
            Color::BRIGHT_MAGENTA => {self.bright_magenta = Some(code)}
            Color::BRIGHT_YELLOW => {self.bright_yellow = Some(code)}
            Color::BRIGHT_WHITE => {self.bright_white = Some(code)}
        }
    }
    pub fn get(&self, color: Color) -> Option<&str> {
        match color{
            Color::BLACK => {self.black}
            Color::BLUE => {self.blue}
            Color::GREEN => {self.green}
            Color::CYAN => {self.cyan}
            Color::RED => {self.red}
            Color::MAGENTA => {self.magenta}
            Color::YELLOW => {self.yellow}
            Color::WHITE => {self.white}
            Color::GRAY => {self.gray}
            Color::BRIGHT_BLUE => {self.bright_blue}
            Color::BRIGHT_GREEN => {self.bright_green}
            Color::BRIGHT_CYAN => {self.bright_cyan}
            Color::BRIGHT_RED => {self.bright_red}
            Color::BRIGHT_MAGENTA => {self.bright_magenta}
            Color::BRIGHT_YELLOW => {self.bright_yellow}
            Color::BRIGHT_WHITE => {self.bright_white}
        }.clone()
    }
    pub fn getdef(&self, color: Color) -> &str {
        match self.get(color) {
            Some(s) => s,
            None => {
                match color{
                    Color::BLACK => default_colors::BLACK,
                    Color::BLUE => default_colors::BLUE,
                    Color::GREEN => default_colors::GREEN,
                    Color::CYAN => default_colors::CYAN,
                    Color::RED => default_colors::RED,
                    Color::MAGENTA => default_colors::MAGENTA,
                    Color::YELLOW => default_colors::YELLOW,
                    Color::WHITE => default_colors::WHITE,
                    Color::GRAY => default_colors::GRAY,
                    Color::BRIGHT_BLUE => default_colors::BRIGHT_BLUE,
                    Color::BRIGHT_GREEN => default_colors::BRIGHT_GREEN,
                    Color::BRIGHT_CYAN => default_colors::BRIGHT_CYAN,
                    Color::BRIGHT_RED => default_colors::BRIGHT_RED,
                    Color::BRIGHT_MAGENTA => default_colors::BRIGHT_MAGENTA,
                    Color::BRIGHT_YELLOW => default_colors::BRIGHT_YELLOW,
                    Color::BRIGHT_WHITE => default_colors::BRIGHT_WHITE,
                }
            }
        }
    }
    pub fn render(&self, prefix: &str, color_prefix: &str) -> String{
        let mut ret = String::new();
        ret = format!("{}.{}black{{{}color:{};}}", ret, prefix, color_prefix, self.getdef(Color::BLACK));
        ret = format!("{}.{}blue{{{}color:{};}}", ret, prefix, color_prefix, self.getdef(Color::BLUE));
        ret = format!("{}.{}green{{{}color:{};}}", ret, prefix, color_prefix, self.getdef(Color::GREEN));
        ret = format!("{}.{}cyan{{{}color:{};}}", ret, prefix, color_prefix, self.getdef(Color::CYAN));
        ret = format!("{}.{}red{{{}color:{};}}", ret, prefix, color_prefix, self.getdef(Color::RED));
        ret = format!("{}.{}magenta{{{}color:{};}}", ret, prefix, color_prefix, self.getdef(Color::MAGENTA));
        ret = format!("{}.{}yellow{{{}color:{};}}", ret, prefix, color_prefix, self.getdef(Color::YELLOW));
        ret = format!("{}.{}white{{{}color:{};}}", ret, prefix, color_prefix, self.getdef(Color::WHITE));
        ret = format!("{}.{}gray{{{}color:{};}}", ret, prefix, color_prefix, self.getdef(Color::GRAY));
        ret = format!("{}.{}bblue{{{}color:{};}}", ret, prefix, color_prefix, self.getdef(Color::BRIGHT_BLUE));
        ret = format!("{}.{}bgreen{{{}color:{};}}", ret, prefix, color_prefix, self.getdef(Color::BRIGHT_GREEN));
        ret = format!("{}.{}bcyan{{{}color:{};}}", ret, prefix, color_prefix, self.getdef(Color::BRIGHT_CYAN));
        ret = format!("{}.{}bred{{{}color:{};}}", ret, prefix, color_prefix, self.getdef(Color::BRIGHT_RED));
        ret = format!("{}.{}bmagenta{{{}color:{};}}", ret, prefix, color_prefix, self.getdef(Color::BRIGHT_MAGENTA));
        ret = format!("{}.{}byellow{{{}color:{};}}", ret, prefix, color_prefix, self.getdef(Color::BRIGHT_YELLOW));
        ret = format!("{}.{}bwhite{{{}color:{};}}", ret, prefix, color_prefix, self.getdef(Color::BRIGHT_WHITE));
        ret
    }
}

#[cfg(test)]
mod palette_tests {
    use crate::Palette;
    #[test]
    fn render() {
        let bp = Palette::new();
        let rf = String::from("\
            .w3a-fg-black{color:black;}\
            .w3a-fg-blue{color:navy;}\
            .w3a-fg-green{color:green;}\
            .w3a-fg-cyan{color:teal;}\
            .w3a-fg-red{color:maroon;}\
            .w3a-fg-magenta{color:purple;}\
            .w3a-fg-yellow{color:olive;}\
            .w3a-fg-white{color:silver;}\
            .w3a-fg-gray{color:gray;}\
            .w3a-fg-bblue{color:blue;}\
            .w3a-fg-bgreen{color:lime;}\
            .w3a-fg-bcyan{color:aqua;}\
            .w3a-fg-bred{color:red;}\
            .w3a-fg-bmagenta{color:fuchsia;}\
            .w3a-fg-byellow{color:yellow;}\
            .w3a-fg-bwhite{color:white;}\
            .w3a-bg-black{background-color:black;}\
            .w3a-bg-blue{background-color:navy;}\
            .w3a-bg-green{background-color:green;}\
            .w3a-bg-cyan{background-color:teal;}\
            .w3a-bg-red{background-color:maroon;}\
            .w3a-bg-magenta{background-color:purple;}\
            .w3a-bg-yellow{background-color:olive;}\
            .w3a-bg-white{background-color:silver;}\
            .w3a-bg-gray{background-color:gray;}\
            .w3a-bg-bblue{background-color:blue;}\
            .w3a-bg-bgreen{background-color:lime;}\
            .w3a-bg-bcyan{background-color:aqua;}\
            .w3a-bg-bred{background-color:red;}\
            .w3a-bg-bmagenta{background-color:fuchsia;}\
            .w3a-bg-byellow{background-color:yellow;}\
            .w3a-bg-bwhite{background-color:white;}\
        ");
        assert_eq!(rf, bp.render("w3a-"));
    }
}

#[cfg(test)]
mod base_palette_tests {
    use crate::BasePalette;
    use crate::default_colors::*;
    use rs3a::Color;
    #[test]
    fn defaults() {
        let bp = BasePalette::new();
        assert_eq!(bp.getdef(Color::BLACK), BLACK);
        assert_eq!(bp.getdef(Color::BLUE), BLUE);
        assert_eq!(bp.getdef(Color::GREEN), GREEN);
        assert_eq!(bp.getdef(Color::CYAN), CYAN);
        assert_eq!(bp.getdef(Color::RED), RED);
        assert_eq!(bp.getdef(Color::MAGENTA), MAGENTA);
        assert_eq!(bp.getdef(Color::YELLOW), YELLOW);
        assert_eq!(bp.getdef(Color::WHITE), WHITE);
        assert_eq!(bp.getdef(Color::GRAY), GRAY);
        assert_eq!(bp.getdef(Color::BRIGHT_BLUE), BRIGHT_BLUE);
        assert_eq!(bp.getdef(Color::BRIGHT_GREEN), BRIGHT_GREEN);
        assert_eq!(bp.getdef(Color::BRIGHT_CYAN), BRIGHT_CYAN);
        assert_eq!(bp.getdef(Color::BRIGHT_RED), BRIGHT_RED);
        assert_eq!(bp.getdef(Color::BRIGHT_MAGENTA), BRIGHT_MAGENTA);
        assert_eq!(bp.getdef(Color::BRIGHT_YELLOW), BRIGHT_YELLOW);
        assert_eq!(bp.getdef(Color::BRIGHT_WHITE), BRIGHT_WHITE);
    }
    #[test]
    fn nondefaults() {
        let mut bp = BasePalette::new();
        bp.set(Color::BLACK, "1");
        bp.set(Color::BLUE, "2");
        bp.set(Color::GREEN, "3");
        bp.set(Color::CYAN, "4");
        bp.set(Color::RED, "5");
        bp.set(Color::MAGENTA, "6");
        bp.set(Color::YELLOW, "7");
        bp.set(Color::WHITE, "8");
        bp.set(Color::GRAY, "9");
        bp.set(Color::BRIGHT_BLUE, "10");
        bp.set(Color::BRIGHT_GREEN, "11");
        bp.set(Color::BRIGHT_CYAN, "12");
        bp.set(Color::BRIGHT_RED, "13");
        bp.set(Color::BRIGHT_MAGENTA, "14");
        bp.set(Color::BRIGHT_YELLOW, "15");
        bp.set(Color::BRIGHT_WHITE, "16");
        assert_eq!(bp.getdef(Color::BLACK), "1");
        assert_eq!(bp.getdef(Color::BLUE), "2");
        assert_eq!(bp.getdef(Color::GREEN), "3");
        assert_eq!(bp.getdef(Color::CYAN), "4");
        assert_eq!(bp.getdef(Color::RED), "5");
        assert_eq!(bp.getdef(Color::MAGENTA), "6");
        assert_eq!(bp.getdef(Color::YELLOW), "7");
        assert_eq!(bp.getdef(Color::WHITE), "8");
        assert_eq!(bp.getdef(Color::GRAY), "9");
        assert_eq!(bp.getdef(Color::BRIGHT_BLUE), "10");
        assert_eq!(bp.getdef(Color::BRIGHT_GREEN), "11");
        assert_eq!(bp.getdef(Color::BRIGHT_CYAN), "12");
        assert_eq!(bp.getdef(Color::BRIGHT_RED), "13");
        assert_eq!(bp.getdef(Color::BRIGHT_MAGENTA), "14");
        assert_eq!(bp.getdef(Color::BRIGHT_YELLOW), "15");
        assert_eq!(bp.getdef(Color::BRIGHT_WHITE), "16");
    }
    #[test]
    fn render() {
        let bp = BasePalette::new();
        let rf = String::from("\
            .w3a-black{color:black;}\
            .w3a-blue{color:navy;}\
            .w3a-green{color:green;}\
            .w3a-cyan{color:teal;}\
            .w3a-red{color:maroon;}\
            .w3a-magenta{color:purple;}\
            .w3a-yellow{color:olive;}\
            .w3a-white{color:silver;}\
            .w3a-gray{color:gray;}\
            .w3a-bblue{color:blue;}\
            .w3a-bgreen{color:lime;}\
            .w3a-bcyan{color:aqua;}\
            .w3a-bred{color:red;}\
            .w3a-bmagenta{color:fuchsia;}\
            .w3a-byellow{color:yellow;}\
            .w3a-bwhite{color:white;}\
        ");
        assert_eq!(rf, bp.render("w3a-", ""));
    }
}
