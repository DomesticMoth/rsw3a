use rs3a::{Color, Art, Frame, ColorMod};
use std::collections::HashMap;
use base64;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::Rng;
#[macro_use]
extern crate lazy_static;

/*
    <TODO>
        - Add invisible characters bg color optimisation to frame_to_commands
        - Add getting the uuid by the render function from the outside instead of generating it internally
        - Make the render an independent function and not a WebRenderer method
        - Test render function on real arts
        - Write system for converting html files with change <3a> tags to css aimations
        - Split the code into separate files
        - Reformat code
        - Add rustfmt & clippy to project
    </TODO>
*/

lazy_static! {
    static ref ALL_COLORS: Vec<Color> = vec![
        Color::BLACK,
        Color::BLUE,
        Color::GREEN,
        Color::CYAN,
        Color::RED,
        Color::MAGENTA,
        Color::YELLOW,
        Color::WHITE,
        Color::GRAY,
        Color::BRIGHT_BLUE,
        Color::BRIGHT_GREEN,
        Color::BRIGHT_CYAN,
        Color::BRIGHT_RED,
        Color::BRIGHT_MAGENTA,
        Color::BRIGHT_YELLOW,
        Color::BRIGHT_WHITE,
    ];
}

macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        core::convert::From::from([$(($k, $v),)*])
    }};
    // set-like
    ($($v:expr),* $(,)?) => {{
        core::convert::From::from([$($v,)*])
    }};
}

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
pub struct Palette {
    pub fg: BasePalette,
    pub bg: BasePalette,
}

impl Palette {
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
pub struct BasePalette {
    black: Option<String>,
    blue: Option<String>,
    green: Option<String>,
    cyan: Option<String>,
    red: Option<String>,
    magenta: Option<String>,
    yellow: Option<String>,
    white: Option<String>,
    gray: Option<String>,
    bright_blue: Option<String>,
    bright_green: Option<String>,
    bright_cyan: Option<String>,
    bright_red: Option<String>,
    bright_magenta: Option<String>,
    bright_yellow: Option<String>,
    bright_white: Option<String>,
}

impl BasePalette {
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
    pub fn set(&mut self, color: Color, code: String) {
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
    pub fn get(&self, color: Color) -> Option<String> {
        match color{
            Color::BLACK => {self.black.clone()}
            Color::BLUE => {self.blue.clone()}
            Color::GREEN => {self.green.clone()}
            Color::CYAN => {self.cyan.clone()}
            Color::RED => {self.red.clone()}
            Color::MAGENTA => {self.magenta.clone()}
            Color::YELLOW => {self.yellow.clone()}
            Color::WHITE => {self.white.clone()}
            Color::GRAY => {self.gray.clone()}
            Color::BRIGHT_BLUE => {self.bright_blue.clone()}
            Color::BRIGHT_GREEN => {self.bright_green.clone()}
            Color::BRIGHT_CYAN => {self.bright_cyan.clone()}
            Color::BRIGHT_RED => {self.bright_red.clone()}
            Color::BRIGHT_MAGENTA => {self.bright_magenta.clone()}
            Color::BRIGHT_YELLOW => {self.bright_yellow.clone()}
            Color::BRIGHT_WHITE => {self.bright_white.clone()}
        }
    }
    pub fn getdef(&self, color: Color) -> String {
        match self.get(color) {
            Some(s) => s,
            None => {
                String::from( match color{
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
                })
            }
        }
    }
    pub fn render(&self, prefix: &str, color_prefix: &str) -> String{
        let mut ret = String::new();
        for color in ALL_COLORS.iter() {
            ret = format!("{}.{}{}{{{}color:{};}}", 
                ret, 
                prefix, 
                color_class_name(*color), 
                color_prefix, 
                self.getdef(*color)
            );
        }
        ret
    }
}

fn color_class_name(color: Color) -> &'static str {
    match color{
        Color::BLACK => "black",
        Color::BLUE => "blue",
        Color::GREEN => "green",
        Color::CYAN => "cyan",
        Color::RED => "red",
        Color::MAGENTA => "magenta",
        Color::YELLOW => "yellow",
        Color::WHITE => "white",
        Color::GRAY => "gray",
        Color::BRIGHT_BLUE => "bblue",
        Color::BRIGHT_GREEN => "bgreen",
        Color::BRIGHT_CYAN => "bcyan",
        Color::BRIGHT_RED => "bred",
        Color::BRIGHT_MAGENTA => "bmagenta",
        Color::BRIGHT_YELLOW => "byellow",
        Color::BRIGHT_WHITE => "bwhite",
    }
}

fn style_render(styles: HashMap<String, String>) -> String {
    let mut ret = String::new();
    for (k, v) in styles {
        ret = format!("{}{}:{};", ret, k, v);
    }
    ret
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct ColorPair {
    fg: Option<Color>,
    bg: Option<Color>,
}

#[derive(Debug, Clone, PartialEq)]
enum Command {
    Print(String),
    SetColor(ColorPair),
}

fn frame_to_commands(frame: &Frame) -> Vec<Command> {
    let mut ret = Vec::new();
    let mut fg: Option<Color> = None;
    let mut bg: Option<Color> = None;
    ret.push(Command::SetColor(ColorPair{fg: None, bg: None}));
    for row in frame {
        for fragment in row {
            if (fragment.fg_color != fg) | (fragment.bg_color != bg) {
                fg = fragment.fg_color;
                bg = fragment.bg_color;
                ret.push(Command::SetColor(ColorPair{fg, bg}));
            }
            ret.push(Command::Print(fragment.text.clone()));
        }
        ret.push(Command::Print("\n".to_string()));
    }
    ret.pop();
    ret
}

fn get_frame_base_color(commands: &Vec<Command>) -> (ColorPair, usize, usize){
    let mut colorpairs: HashMap<ColorPair, (usize, usize)> = HashMap::new();
    let mut current = ColorPair{fg: None, bg: None};
    for cmd in commands {
        match cmd {
            Command::SetColor(cp) => {
                match colorpairs.clone().get(&cp) {
                    Some((mv, ln)) => {
                        colorpairs.insert(*cp, (mv+1, *ln));
                    }
                    None => {
                        colorpairs.insert(*cp, (1, 0));
                    }
                }
                current = *cp;
            }
            Command::Print(text) => {
                match colorpairs.clone().get(&current) {
                    Some((mv, ln)) => {
                        colorpairs.insert(current, (*mv, ln+text.len()));
                    }
                    None => {
                        colorpairs.insert(current, (1, text.len()));
                    }
                }
            }
        }
    }
    let mut cp = ColorPair{fg: None, bg: None};
    let mut mv: usize = 0;
    let mut ln: usize = 0;
    for (c, (m, l)) in colorpairs {
        if (m > mv) || (m == mv && l > ln) {
            cp = c;
            mv = m;
            ln = l;
        }
    }
    (cp, mv, ln)
}

#[derive(Debug, Clone, PartialEq)]
struct PRFrame {
    commands: Vec<Command>,
    base_fg: Option<Color>,
    base_bg: Option<Color>,
}

#[derive(Debug, Clone, PartialEq)]
struct PRArt {
    frames: Vec<PRFrame>,
    base_fg: Option<Color>,
    base_bg: Option<Color>,
}

impl PRArt {
    fn from_art(art: &Art) -> Self {
        let mut colorpairs: HashMap<ColorPair, (usize, usize)> = HashMap::new();
        let mut frames: Vec<PRFrame> = vec![];
        for frame in &art.body.frames {
            let commands = frame_to_commands(frame);
            let (cp, mv, ln) = get_frame_base_color(&commands);
            colorpairs.insert(cp, (mv, ln));
            frames.push(
                PRFrame{
                    commands,
                    base_fg: cp.fg,
                    base_bg: cp.bg,
                }
            );
        }
        let mut cp = ColorPair{fg: None, bg: None};
        let mut mv: usize = 0;
        let mut ln: usize = 0;
        for (c, (m, l)) in colorpairs {
            if (m > mv) || (m == mv && l > ln) {
                cp = c;
                mv = m;
                ln = l;
            }
        }
        PRArt{
            frames,
            base_fg: cp.fg,
            base_bg: cp.bg,
        }
    }
}

fn update_palette_from_styles(mut palette: Palette, styles: HashMap<String, String>) -> Palette {
    for color in ALL_COLORS.iter() {
        if let Some(code) = styles.get(&format!("fg-color-{}", color_class_name(*color))) {
            palette.fg.set(*color, code.clone());
        }
        if let Some(code) = styles.get(&format!("bg-color-{}", color_class_name(*color))) {
            palette.bg.set(*color, code.clone());
        }
    }
    palette
}

#[derive(Debug, Clone, Copy)]
enum ColorType {
    Fg(Option<Color>),
    Bg(Option<Color>),
}

fn colorise(color: ColorType, color_mod: ColorMod, palette: &Palette, independent: bool, nonecolor: bool, prefix: &str, style: &mut HashMap<String, String>, class: &mut String) {
    match color{
        ColorType::Fg(col) => {
            match col {
                None => { if nonecolor { style.insert("color".to_string(), "none".to_string()); } }
                Some(col) => {
                    match color_mod {
                        ColorMod::Fg => {
                            match palette.fg.get(col) {
                                None => {
                                    if independent {
                                        style.insert("color".to_string(), palette.fg.getdef(col).to_string());
                                    }else{
                                        *class = format!("{}{}fg-{} ", class, prefix, color_class_name(col));
                                    }
                                }
                                Some(code) => { style.insert("color".to_string(), code.to_string()); }
                            }
                        }
                        ColorMod::Full => {
                            match palette.fg.get(col) {
                                None => {
                                    if independent {
                                        style.insert("color".to_string(), palette.fg.getdef(col).to_string());
                                    }else{
                                        *class = format!("{}{}fg-{} ", class, prefix, color_class_name(col));
                                    }
                                }
                                Some(code) => { style.insert("color".to_string(), code.to_string()); }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        ColorType::Bg(col) => {
            match col {
                None => { if nonecolor { style.insert("background-color".to_string(), "none".to_string()); } }
                Some(col) => {
                    match color_mod {
                        ColorMod::Bg => {
                            match palette.bg.get(col) {
                                None => {
                                    if independent {
                                        style.insert("background-color".to_string(), palette.bg.getdef(col).to_string());
                                    }else{
                                        *class = format!("{}{}bg-{} ", class, prefix, color_class_name(col));
                                    }
                                }
                                Some(code) => { style.insert("background-color".to_string(), code.to_string()); }
                            }
                        }
                        ColorMod::Full => {
                            match palette.bg.get(col) {
                                None => {
                                    if independent {
                                        style.insert("background-color".to_string(), palette.bg.getdef(col).to_string());
                                    }else{
                                        *class = format!("{}{}bg-{} ", class, prefix, color_class_name(col));
                                    }
                                }
                                Some(code) => { style.insert("background-color".to_string(), code.to_string()); }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn escape(s: String) -> String {
    let mut ret = String::new();
    for c in s.chars() {
        match c {
            '&' => { ret = format!("{}&amp;", ret) }
            '<' => { ret = format!("{}&lt;", ret) }
            '>' => { ret = format!("{}&gt;", ret) }
            c => { ret = format!("{}{}", ret, c) }
        }
    }
    ret
}

pub struct WebRenderer {
    rng: StdRng,
}

impl WebRenderer {
    pub fn new() -> Self {
        Self{ rng: StdRng::seed_from_u64(0) }
    }
    fn uid(&mut self) -> String {
        base64::encode(self.rng.gen::<u128>().to_be_bytes())
    }
    pub fn render(&mut self, art: &Art, palette: Option<Palette>, styles: Option<HashMap<String, String>>, prefix: &str) -> String {
        let styles: HashMap<String, String> = match styles {
            Some(styles) => styles,
            None => { HashMap::new() }
        };
        let uid = self.uid();
        let preart = PRArt::from_art(art);
        let mut independent: bool = false;
        let mut class = String::new();
        let mut css: HashMap<String, String> = collection!{
            "display".to_string() => "inline-block".to_string(),
            "position".to_string() => "relative".to_string(),
        };
        for (k, v) in styles { css.insert(k, v); }
        let palette: Palette = update_palette_from_styles(match palette {
            Some(palette) => palette,
            None => {Palette::new()}
        }, css.clone());
        let mut loop_enable = art.header.loop_enable;
        if let Some(v) = css.get(&"independent".to_string()) {
            if v == "true" {
                independent = true;
            }else if v == "false" {
                independent = false;
            }
        }
        if let Some(v) = css.get(&"loop-animation".to_string()) {
            if v == "true" {
                loop_enable = true;
            }else if v == "false" {
                loop_enable = false;
            }
        }
        let mut preview_frame = art.header.preview;
        if let Some(v) = css.get(&"preview-frame".to_string()) {
            match v.parse::<u16>() {
                Ok(v) => { preview_frame = v; }
                Err(_) => {}
            }
        }
        if preview_frame as usize > art.body.frames.len() {
            preview_frame = 0;
        }
        let mut only_preview = false;
        if let Some(v) = css.get(&"only-preview".to_string()) {
            if v == "true" {
                only_preview = true;
            }else if v == "false" {
                only_preview = false;
            }
        }
        let mut color_mod = art.header.color_mod;
        if let Some(md) = css.get(&"color-mod".to_string()) {
            match md.as_str() {
                "none" => { color_mod = ColorMod::None }
                "fg" =>   { color_mod = ColorMod::Fg }
                "bg" =>   { color_mod = ColorMod::Bg }
                "full" => { color_mod = ColorMod::Full }
                _ => {}
            }
        }
        css.insert("background-color".to_string(), "none".to_string());
        css.insert("color".to_string(), "none".to_string());
        colorise(ColorType::Fg(preart.base_fg), color_mod, &palette, independent, true, prefix, &mut css, &mut class);
        colorise(ColorType::Bg(preart.base_bg), color_mod, &palette, independent, true, prefix, &mut css, &mut class);
        let mut anim_styles = String::new();
        if !only_preview {
            let anim_time = (art.body.frames.len() as f64 * art.header.delay as f64) / 1000.0;
            for i in 0..art.body.frames.len() {
                let stp: f64 = (100.0 / art.body.frames.len() as f64) * i as f64;
                let mp: f64 = (100.0 / art.body.frames.len() as f64) * (i as f64 + 0.5);
                let ep: f64 = (100.0 / art.body.frames.len() as f64) * (i + 1) as f64;
                anim_styles = format!("{}\
                    .{uid}-f{nom}{{animation:{uid}-f{nom}a {time}{inf};}}\
                    @keyframes {uid}-f{nom}a{{{stp}%{{visibility:hidden;}}{mp}%{{visibility:visible;}}{ep}%{{visibility:hidden;}}}}\
                    ", anim_styles, nom = i, uid = uid, time = anim_time, stp = stp, mp = mp, ep = ep,
                    inf = match loop_enable {
                        true => " infinite",
                        false => " forwards",
                    });
            }
        }
        let all_styles = format!("\
            <style type=\"text/css\">\
            .{uid}{{visibility:{vis};display:inline-block;top:0;left:0;margin:0;}}\
            {anim}\
            </style>\
        ", uid = uid, anim = anim_styles, vis = if only_preview {"visible"}else{"hidden"});
        let mut frames = String::new();
        for (nom, frame) in preart.frames.iter().enumerate() {
            if only_preview {
                if nom != preview_frame as usize { continue };
            }
            let mut frame_classes = format!("{uid} {uid}-f{nom} ", uid = uid, nom = nom);
            let mut frame_styles = HashMap::<String, String>::new();
            if nom > 0 {
                frame_styles.insert("position".to_string(), "absolute".to_string());
            }
            // add color styles
            if frame.base_fg != preart.base_fg {
                colorise(ColorType::Fg(frame.base_fg), color_mod, &palette, independent, true, prefix, &mut frame_styles, &mut frame_classes);
            }
            if frame.base_bg != preart.base_bg {
                colorise(ColorType::Bg(frame.base_bg), color_mod, &palette, independent, true, prefix, &mut frame_styles, &mut frame_classes);
            }
            let frame_styles = if frame_styles.len() > 0 {
                format!(" style=\"{}\"", style_render(frame_styles))
            }else{ String::new() };
            frames = format!("{}<pre class=\"{}\"{}>\n", frames, frame_classes, frame_styles);
            // render frame body
            let mut span = false;
            for cmd in &frame.commands {
                match cmd {
                    Command::Print(text) => {
                        frames = format!("{}{}", frames, escape(text.to_string()));
                    }
                    Command::SetColor(pair) => {
                        if span {
                            frames = format!("{}</span>", frames);
                            span = false;
                        }
                        if pair.fg != frame.base_fg || pair.bg != frame.base_bg {
                            let mut span_classes = String::new();
                            let mut span_styles = HashMap::<String, String>::new();
                            colorise(ColorType::Fg(pair.fg), color_mod, &palette, independent, true, prefix, &mut span_styles, &mut span_classes);
                            colorise(ColorType::Bg(pair.bg), color_mod, &palette, independent, true, prefix, &mut span_styles, &mut span_classes);
                            if span_styles.len() + span_classes.len() > 0 {
                                span = true;
                                let span_classes = if span_classes.len() > 0 {
                                    format!(" class=\"{}\"", span_classes)
                                }else{ String::new() };
                                let span_styles = if span_styles.len() > 0 {
                                    format!(" style=\"{}\"", style_render(span_styles))
                                }else{ String::new() };
                                frames = format!("{}<span{}{}>", frames, span_classes, span_styles);
                            }
                        }
                    }
                }
            }
            frames = format!("{}</pre>\n", frames);
        }
        let title = match &art.header.title {
            Some(title) => { format!(" title=\"{}\"", title) }
            None => { String::new() }
        };
        let author = match &art.header.author {
            Some(author) => { format!(" author=\"{}\"", author) }
            None => { String::new() }
        };
        format!("<div class=\"{}\" style=\"{}\"{}{}>\n{}\n{}</div>", class, style_render(css), title, author, all_styles, frames)
    }
}

#[cfg(test)]
mod render_tests {
    use rs3a;
    use crate::*;
    const APPLE: &str = r###"@
@ This work is licensed under the Creative Commons Attribution 4.0 International License. To view a copy of this license, visit
@ http://creativecommons.org/licenses/by/4.0/.
title just an apple
author DomesticMoth
width 12
height 5
loop true
delay 300
colors fg

  ,--./,-.  444444444444
 / //     \ 444cc4444444
|          |444444444444
 \        / 444444444444
  '._,._,'  444444444444

  ,--./,-.  444444444444
 / //    _\ 444cc4444444
|       /   4444444ffff4
 \      `-, 4444444ffff4
  '._,._,'  444444444444

  ,--./,-.  444444444444
 / //   ,-' 444cc4444444
|      (    4444444f4444
 \      `-, 4444444ffff4
  '._,._,'  444444444444

  ,--./,-.  444444444444
 / //,_.--' 444cc4444444
|   /  {    4444fffffff4
 \  \_,-`-, 4444fffffff4
  '._,._,'  444444444444

  ,--./,-.  444444444444
 /,-._,.--' 444444444444
  __}  {    44fffffffff4
 \`-._,-`-, 44fffffffff4
  '._,._,'  444444444444

  ,--./,-.  444444444444
 '--._,.--' 444444444444
    }  {    fffffffffff4
 ,-'._,-`-, fffffffffff4
  '._,._,'  444444444444
    "###;
    #[test]
    fn test_apple(){
        let apple = rs3a::load(APPLE.to_string()).unwrap();
        let render = WebRenderer::new().render(&apple, None, None, "prefix-");
        assert_eq!(render, String::new());
    }
}

#[cfg(test)]
mod other_tests {
    use crate::{escape, ColorType, colorise, Palette};
    use std::collections::HashMap;
    use rs3a::{ColorMod,Color};
    struct ColoriseTestCase {
        color: ColorType,
        color_mod: ColorMod,
        palette: Palette,
        independent: bool,
        nonecolor: bool,
        prefix: String,
        style: HashMap<String, String>,
        class: String,
    }
    fn colorise_cases_test(cases: Vec<ColoriseTestCase>){
        for case in cases {
            let mut ret_style = HashMap::<String, String>::new();
            let mut ret_class = String::new();
            colorise(
                case.color,
                case.color_mod,
                &case.palette,
                case.independent,
                case.nonecolor,
                &case.prefix,
                &mut ret_style,
                &mut ret_class
            );
            assert_eq!(ret_style, case.style);
            assert_eq!(ret_class, case.class);
        }
    }
    #[test]
    fn test_colorise() {
        let mut palette = Palette::new();
        palette.fg.set(Color::RED, "reeeeeeed".to_string());
        palette.bg.set(Color::RED, "reeeeeeed".to_string());
        let prefix = String::from("prefix-");
        colorise_cases_test(vec![
            ColoriseTestCase {
                color: ColorType::Fg(Some(Color::BLACK)),
                color_mod: ColorMod::Full,
                palette: palette.clone(),
                independent: true,
                nonecolor: true,
                prefix: prefix.clone(),
                style: collection!{
                    "color".to_string() => "black".to_string(),
                },
                class: String::from(""),
            },
            ColoriseTestCase {
                color: ColorType::Fg(Some(Color::BLACK)),
                color_mod: ColorMod::Full,
                palette: palette.clone(),
                independent: false,
                nonecolor: true,
                prefix: prefix.clone(),
                style: HashMap::new(),
                class: String::from("prefix-fg-black "),
            },
            ColoriseTestCase {
                color: ColorType::Fg(Some(Color::RED)),
                color_mod: ColorMod::Full,
                palette: palette.clone(),
                independent: true,
                nonecolor: true,
                prefix: prefix.clone(),
                style: collection!{
                    "color".to_string() => "reeeeeeed".to_string(),
                },
                class: String::from(""),
            },
            ColoriseTestCase {
                color: ColorType::Fg(None),
                color_mod: ColorMod::Full,
                palette: palette.clone(),
                independent: true,
                nonecolor: false,
                prefix: prefix.clone(),
                style: HashMap::new(),
                class: String::from(""),
            },
            ColoriseTestCase {
                color: ColorType::Fg(None),
                color_mod: ColorMod::Full,
                palette: palette.clone(),
                independent: false,
                nonecolor: false,
                prefix: prefix.clone(),
                style: HashMap::new(),
                class: String::from(""),
            },
            ColoriseTestCase {
                color: ColorType::Fg(None),
                color_mod: ColorMod::Full,
                palette: palette.clone(),
                independent: true,
                nonecolor: true,
                prefix: prefix.clone(),
                style: collection!{
                    "color".to_string() => "none".to_string(),
                },
                class: String::from(""),
            },
            ColoriseTestCase {
                color: ColorType::Fg(None),
                color_mod: ColorMod::Full,
                palette: palette.clone(),
                independent: false,
                nonecolor: true,
                prefix: prefix.clone(),
                style: collection!{
                    "color".to_string() => "none".to_string(),
                },
                class: String::from(""),
            },
        ]);
    }
    #[test]
    fn test_escape() {
        assert_eq!(
            String::from("&lt;js&gt; inject!(&amp;&amp;&amp;&amp;&amp; текст); &lt;/js&gt;"),
            escape(String::from("<js> inject!(&&&&& текст); </js>"))
        );
    }
}

#[cfg(test)]
mod palette_tests {
    use crate::Palette;
    #[test]
    fn test_render() {
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
    fn test_defaults() {
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
    fn test_nondefaults() {
        let mut bp = BasePalette::new();
        bp.set(Color::BLACK, "1".into());
        bp.set(Color::BLUE, "2".into());
        bp.set(Color::GREEN, "3".into());
        bp.set(Color::CYAN, "4".into());
        bp.set(Color::RED, "5".into());
        bp.set(Color::MAGENTA, "6".into());
        bp.set(Color::YELLOW, "7".into());
        bp.set(Color::WHITE, "8".into());
        bp.set(Color::GRAY, "9".into());
        bp.set(Color::BRIGHT_BLUE, "10".into());
        bp.set(Color::BRIGHT_GREEN, "11".into());
        bp.set(Color::BRIGHT_CYAN, "12".into());
        bp.set(Color::BRIGHT_RED, "13".into());
        bp.set(Color::BRIGHT_MAGENTA, "14".into());
        bp.set(Color::BRIGHT_YELLOW, "15".into());
        bp.set(Color::BRIGHT_WHITE, "16".into());
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
    fn test_render() {
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
