use rs3a::{Color, Art, Frame, ColorMod};
use std::collections::HashMap;
//use base64;
//use rand::rngs::StdRng;
//use rand::SeedableRng;
//use rand::Rng;
#[macro_use]
extern crate lazy_static;

/*
    <TODO>
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
    let mut sorted: Vec<_> = styles.iter().collect();
    sorted.sort();
    for (k, v) in sorted.iter() {
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

fn is_void(s: &str) -> bool {
    let mut ret = true;
    for c in s.chars() {
        if !(c == ' ' || c == '\t') {
            ret = false;
        }
    }
    ret
}

fn frame_to_commands(frame: &Frame) -> Vec<Command> {
    let mut ret = Vec::new();
    let mut fg: Option<Color> = None;
    let mut bg: Option<Color> = None;
    let mut unset = true;
    for row in frame {
        for fragment in row {
            if fragment.fg_color != fg || fragment.bg_color != bg || unset {
                if !is_void(&fragment.text) || fragment.bg_color != bg {
                    fg = fragment.fg_color;
                    bg = fragment.bg_color;
                    ret.push(Command::SetColor(ColorPair{fg, bg}));
                    unset = false;
                }
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

#[cfg(test)]
mod peart_tests {
    use rs3a;
    use crate::*;
    const EXAMPLE : &str = r###"
	Header starts here
	Comments starts with tab char
width 22	Count of symbols in column
height 14	Count of rows in frames
loop true
colors full	Colors are specified for both text and background
delay 300
title 3a demo
author Moth
@ In header comments also may starts with @ char



	There
	is
	one
	or
	more
	empty
	lines
	between
	header
	and
	body



	Body starts here
	First frame
LU]Pk&3):F*k[]qbd;$0Bp77777777777777777777770000000000000000000000	First row
}YZ7Ik;=a^KF(0CxvF5AU+77777777777777777777770000000000000000000000	Second row
uPJZ]RJ3]^xsyJ~-;2~.dW777777cccccc77777777770000003333330000000000	Third row
pj&<rH.vFN6odJ5c-l~CRx777777cccccc77777777770000003333330000000000
A!ht(UB@(jNDl.67+n)?,N7777777777cc77cccccc770000000000330033333300
v6,gTX64AWFW>%>IdAxSgW7777777777cc77cccccc770000000000330033333300
}[FiPQzkcMdG@K!<@dvC-$777777cccccc77cc77cc770000003333330033003300
C:DT+Odt-P(0pu%r}vlr#H777777cccccc77cc77cc770000003333330033003300
wAD)<iMp>L}yh}Y+}-r$BN7777777777cc77cccccc770000000000330033333300
@,ekE:w[Xt8sCmxxFj9EHN7777777777cc77cccccc770000000000330033333300
rX7=W;uEZii2*xosZuDRBg77cc77cccccc77cc77cc770033003333330033003300
vHtkD411dJ>P~3=kkB4wU^77cc77cccccc77cc77cc770033003333330033003300
,6oFB+gD3f_%gJua{50=HD77777777777777777777770000000000000000000000
rq5?T0lL06Vg-[0G:,sum)77777777777777777777770000000000000000000000

	Second frame
lH<^&?@^Hbt^3v5]7gx9<o	First column in a first row
7777777777777777777777	Second column in a first row
0000000000000000000000	Third column in a first row
hoB<mg>DGKGfM3woWh4%EC77777777777777777777770000000000000000000000
Js%6!JG[m,C8PVsrG7_ESx777777cccccc77777777770000003333330000000000
dCX<E6]xxP~Hk(([D*dGU?777777cccccc77777777770000003333330000000000
~!gdx`w[f0z)xKbnj{rAWS7777777777cc77cccccc770000000000330033333300
5V=hf%`00gl2-YlT;L*&U`7777777777cc77cccccc770000000000330033333300
qk[3_hDMUNNIn}Y`F>lh@o777777cccccc77cc77cc770000003333330033003300
HsHD0=38OO.#iCA&~U[{kT777777cccccc77cc77cc77
0000003333330033003300v#x@h.OzV,zLI6#5&kv4T-777777777	You can add line breaks anywhere in the body.
7cc77cccccc770000000000330033333300			Any way they, like comments, are ignored by the parser.
#t:Gic$*w-i;P<O!fIWdC,7777777777cc77cccccc770000000000330033333300
zs{lk^a3Ty8SbYgLGe7Pdt77cc77cccccc77cc77cc770033003333330033003300
i#%5(w~u9+cdlOdO!&Ms}677cc77cccccc77cc77cc770033003333330033003300
Ou=JC8Zn.T;pq98cx)ov>~77777777777777777777770000000000000000000000
aNvuQi.U6%7-Kf,uk{FG[J77777777777777777777770000000000000000000000

	Third frame
Jk%bTX:]aZ,D?jkB0?*I*O77777777777777777777770000000000000000000000
r~&Em{~S%FVCv._]xdt5.,77777777777777777777770000000000000000000000
OEr=o[s:ocqCa;,h2a-d:q777777cccccc77777777770000003333330000000000
U6]~]fX>~%T%(RAa$p`~n!777777cccccc77777777770000003333330000000000
YRq`hu=gRKg.!k>82v#[^D7777777777cc77cccccc770000000000330033333300
C#F7Nq3U3!yopFgBSRoUPB7777777777cc77cccccc770000000000330033333300
)p_U9Fhxi]W1IE)=s$>dh^777777cccccc77cc77cc770000003333330033003300
8uhRPgAy2}uioTM5Rw@>*5777777cccccc77cc77cc770000003333330033003300
xgq#T=V5d}8WaQU+kez]>X7777777777cc77cccccc770000000000330033333300
ed`pS6%DK9N% iQo7-[gDO7777777777cc77cccccc770000000000330033333300
[Z:jGUY%L&$74[@Q8;Km~E77cc77cccccc77cc77cc770033003333330033003300
gQ{ikr-5fyM<{ny6=]r4U$77cc77cccccc77cc77cc770033003333330033003300
iVG.vOv5uWkulYY#GT[&Tm77777777777777777777770000000000000000000000
U0DC_D-@ml4[7sP7&)C9Q>77777777777777777777770000000000000000000000
    "###;
    const VALID_EXAMPLE: &str = "<div class=\"PREFIXfg-white PREFIXbg-black \" style=\"display:inline-block;position:relative;\" title=\"3a demo\" author=\"Moth\">\n<style type=\"text/css\">.EXAMPLE{visibility:hidden;display:inline-block;top:0;left:0;margin:0;}.EXAMPLE-f0{animation:EXAMPLE-f0a 0.9s infinite;}@keyframes EXAMPLE-f0a{0%{visibility:hidden;}16.666666666666668%{visibility:visible;}33.333333333333336%{visibility:hidden;}}.EXAMPLE-f1{animation:EXAMPLE-f1a 0.9s infinite;}@keyframes EXAMPLE-f1a{33.333333333333336%{visibility:hidden;}50%{visibility:visible;}66.66666666666667%{visibility:hidden;}}.EXAMPLE-f2{animation:EXAMPLE-f2a 0.9s infinite;}@keyframes EXAMPLE-f2a{66.66666666666667%{visibility:hidden;}83.33333333333334%{visibility:visible;}100%{visibility:hidden;}}</style>\n<pre class=\"EXAMPLE EXAMPLE-f0 \">\nLU]Pk&amp;3):F*k[]qbd;$0Bp\n}YZ7Ik;=a^KF(0CxvF5AU+\nuPJZ]R<span class=\"PREFIXfg-bred PREFIXbg-cyan \">J3]^xs</span>yJ~-;2~.dW\npj&amp;&lt;rH<span class=\"PREFIXfg-bred PREFIXbg-cyan \">.vFN6o</span>dJ5c-l~CRx\nA!ht(UB@(j<span class=\"PREFIXfg-bred PREFIXbg-cyan \">ND</span>l.<span class=\"PREFIXfg-bred PREFIXbg-cyan \">67+n)?</span>,N\nv6,gTX64AW<span class=\"PREFIXfg-bred PREFIXbg-cyan \">FW</span>&gt;%<span class=\"PREFIXfg-bred PREFIXbg-cyan \">&gt;IdAxS</span>gW\n}[FiPQ<span class=\"PREFIXfg-bred PREFIXbg-cyan \">zkcMdG</span>@K<span class=\"PREFIXfg-bred PREFIXbg-cyan \">!&lt;</span>@d<span class=\"PREFIXfg-bred PREFIXbg-cyan \">vC</span>-$\nC:DT+O<span class=\"PREFIXfg-bred PREFIXbg-cyan \">dt-P(0</span>pu<span class=\"PREFIXfg-bred PREFIXbg-cyan \">%r</span>}v<span class=\"PREFIXfg-bred PREFIXbg-cyan \">lr</span>#H\nwAD)&lt;iMp&gt;L<span class=\"PREFIXfg-bred PREFIXbg-cyan \">}y</span>h}<span class=\"PREFIXfg-bred PREFIXbg-cyan \">Y+}-r$</span>BN\n@,ekE:w[Xt<span class=\"PREFIXfg-bred PREFIXbg-cyan \">8s</span>Cm<span class=\"PREFIXfg-bred PREFIXbg-cyan \">xxFj9E</span>HN\nrX<span class=\"PREFIXfg-bred PREFIXbg-cyan \">7=</span>W;<span class=\"PREFIXfg-bred PREFIXbg-cyan \">uEZii2</span>*x<span class=\"PREFIXfg-bred PREFIXbg-cyan \">os</span>Zu<span class=\"PREFIXfg-bred PREFIXbg-cyan \">DR</span>Bg\nvH<span class=\"PREFIXfg-bred PREFIXbg-cyan \">tk</span>D4<span class=\"PREFIXfg-bred PREFIXbg-cyan \">11dJ&gt;P</span>~3<span class=\"PREFIXfg-bred PREFIXbg-cyan \">=k</span>kB<span class=\"PREFIXfg-bred PREFIXbg-cyan \">4w</span>U^\n,6oFB+gD3f_%gJua{50=HD\nrq5?T0lL06Vg-[0G:,sum)</pre>\n<pre class=\"EXAMPLE EXAMPLE-f1 \" style=\"position:absolute;\">\nlH&lt;^&amp;?@^Hbt^3v5]7gx9&lt;o\nhoB&lt;mg&gt;DGKGfM3woWh4%EC\nJs%6!J<span class=\"PREFIXfg-bred PREFIXbg-cyan \">G[m,C8</span>PVsrG7_ESx\ndCX&lt;E6<span class=\"PREFIXfg-bred PREFIXbg-cyan \">]xxP~H</span>k(([D*dGU?\n~!gdx`w[f0<span class=\"PREFIXfg-bred PREFIXbg-cyan \">z)</span>xK<span class=\"PREFIXfg-bred PREFIXbg-cyan \">bnj{rA</span>WS\n5V=hf%`00g<span class=\"PREFIXfg-bred PREFIXbg-cyan \">l2</span>-Y<span class=\"PREFIXfg-bred PREFIXbg-cyan \">lT;L*&amp;</span>U`\nqk[3_h<span class=\"PREFIXfg-bred PREFIXbg-cyan \">DMUNNI</span>n}<span class=\"PREFIXfg-bred PREFIXbg-cyan \">Y`</span>F&gt;<span class=\"PREFIXfg-bred PREFIXbg-cyan \">lh</span>@o\nHsHD0=<span class=\"PREFIXfg-bred PREFIXbg-cyan \">38OO.#</span>iC<span class=\"PREFIXfg-bred PREFIXbg-cyan \">A&amp;</span>~U<span class=\"PREFIXfg-bred PREFIXbg-cyan \">[{</span>kT\nv#x@h.OzV,<span class=\"PREFIXfg-bred PREFIXbg-cyan \">zL</span>I6<span class=\"PREFIXfg-bred PREFIXbg-cyan \">#5&amp;kv4</span>T-\n#t:Gic$*w-<span class=\"PREFIXfg-bred PREFIXbg-cyan \">i;</span>P&lt;<span class=\"PREFIXfg-bred PREFIXbg-cyan \">O!fIWd</span>C,\nzs<span class=\"PREFIXfg-bred PREFIXbg-cyan \">{l</span>k^<span class=\"PREFIXfg-bred PREFIXbg-cyan \">a3Ty8S</span>bY<span class=\"PREFIXfg-bred PREFIXbg-cyan \">gL</span>Ge<span class=\"PREFIXfg-bred PREFIXbg-cyan \">7P</span>dt\ni#<span class=\"PREFIXfg-bred PREFIXbg-cyan \">%5</span>(w<span class=\"PREFIXfg-bred PREFIXbg-cyan \">~u9+cd</span>lO<span class=\"PREFIXfg-bred PREFIXbg-cyan \">dO</span>!&amp;<span class=\"PREFIXfg-bred PREFIXbg-cyan \">Ms</span>}6\nOu=JC8Zn.T;pq98cx)ov&gt;~\naNvuQi.U6%7-Kf,uk{FG[J</pre>\n<pre class=\"EXAMPLE EXAMPLE-f2 \" style=\"position:absolute;\">\nJk%bTX:]aZ,D?jkB0?*I*O\nr~&amp;Em{~S%FVCv._]xdt5.,\nOEr=o[<span class=\"PREFIXfg-bred PREFIXbg-cyan \">s:ocqC</span>a;,h2a-d:q\nU6]~]f<span class=\"PREFIXfg-bred PREFIXbg-cyan \">X&gt;~%T%</span>(RAa$p`~n!\nYRq`hu=gRK<span class=\"PREFIXfg-bred PREFIXbg-cyan \">g.</span>!k<span class=\"PREFIXfg-bred PREFIXbg-cyan \">&gt;82v#[</span>^D\nC#F7Nq3U3!<span class=\"PREFIXfg-bred PREFIXbg-cyan \">yo</span>pF<span class=\"PREFIXfg-bred PREFIXbg-cyan \">gBSRoU</span>PB\n)p_U9F<span class=\"PREFIXfg-bred PREFIXbg-cyan \">hxi]W1</span>IE<span class=\"PREFIXfg-bred PREFIXbg-cyan \">)=</span>s$<span class=\"PREFIXfg-bred PREFIXbg-cyan \">&gt;d</span>h^\n8uhRPg<span class=\"PREFIXfg-bred PREFIXbg-cyan \">Ay2}ui</span>oT<span class=\"PREFIXfg-bred PREFIXbg-cyan \">M5</span>Rw<span class=\"PREFIXfg-bred PREFIXbg-cyan \">@&gt;</span>*5\nxgq#T=V5d}<span class=\"PREFIXfg-bred PREFIXbg-cyan \">8W</span>aQ<span class=\"PREFIXfg-bred PREFIXbg-cyan \">U+kez]</span>&gt;X\ned`pS6%DK9<span class=\"PREFIXfg-bred PREFIXbg-cyan \">N%</span> i<span class=\"PREFIXfg-bred PREFIXbg-cyan \">Qo7-[g</span>DO\n[Z<span class=\"PREFIXfg-bred PREFIXbg-cyan \">:j</span>GU<span class=\"PREFIXfg-bred PREFIXbg-cyan \">Y%L&amp;$7</span>4[<span class=\"PREFIXfg-bred PREFIXbg-cyan \">@Q</span>8;<span class=\"PREFIXfg-bred PREFIXbg-cyan \">Km</span>~E\ngQ<span class=\"PREFIXfg-bred PREFIXbg-cyan \">{i</span>kr<span class=\"PREFIXfg-bred PREFIXbg-cyan \">-5fyM&lt;</span>{n<span class=\"PREFIXfg-bred PREFIXbg-cyan \">y6</span>=]<span class=\"PREFIXfg-bred PREFIXbg-cyan \">r4</span>U$\niVG.vOv5uWkulYY#GT[&amp;Tm\nU0DC_D-@ml4[7sP7&amp;)C9Q&gt;</pre>\n</div>";
    const VALID_EXAMPLE_INDEPENDENT: &str = "<div class=\"\" style=\"background-color:black;color:silver;display:inline-block;independent:true;position:relative;\" title=\"3a demo\" author=\"Moth\">\n<style type=\"text/css\">.EXAMPLE{visibility:hidden;display:inline-block;top:0;left:0;margin:0;}.EXAMPLE-f0{animation:EXAMPLE-f0a 0.9s infinite;}@keyframes EXAMPLE-f0a{0%{visibility:hidden;}16.666666666666668%{visibility:visible;}33.333333333333336%{visibility:hidden;}}.EXAMPLE-f1{animation:EXAMPLE-f1a 0.9s infinite;}@keyframes EXAMPLE-f1a{33.333333333333336%{visibility:hidden;}50%{visibility:visible;}66.66666666666667%{visibility:hidden;}}.EXAMPLE-f2{animation:EXAMPLE-f2a 0.9s infinite;}@keyframes EXAMPLE-f2a{66.66666666666667%{visibility:hidden;}83.33333333333334%{visibility:visible;}100%{visibility:hidden;}}</style>\n<pre class=\"EXAMPLE EXAMPLE-f0 \">\nLU]Pk&amp;3):F*k[]qbd;$0Bp\n}YZ7Ik;=a^KF(0CxvF5AU+\nuPJZ]R<span style=\"background-color:teal;color:red;\">J3]^xs</span>yJ~-;2~.dW\npj&amp;&lt;rH<span style=\"background-color:teal;color:red;\">.vFN6o</span>dJ5c-l~CRx\nA!ht(UB@(j<span style=\"background-color:teal;color:red;\">ND</span>l.<span style=\"background-color:teal;color:red;\">67+n)?</span>,N\nv6,gTX64AW<span style=\"background-color:teal;color:red;\">FW</span>&gt;%<span style=\"background-color:teal;color:red;\">&gt;IdAxS</span>gW\n}[FiPQ<span style=\"background-color:teal;color:red;\">zkcMdG</span>@K<span style=\"background-color:teal;color:red;\">!&lt;</span>@d<span style=\"background-color:teal;color:red;\">vC</span>-$\nC:DT+O<span style=\"background-color:teal;color:red;\">dt-P(0</span>pu<span style=\"background-color:teal;color:red;\">%r</span>}v<span style=\"background-color:teal;color:red;\">lr</span>#H\nwAD)&lt;iMp&gt;L<span style=\"background-color:teal;color:red;\">}y</span>h}<span style=\"background-color:teal;color:red;\">Y+}-r$</span>BN\n@,ekE:w[Xt<span style=\"background-color:teal;color:red;\">8s</span>Cm<span style=\"background-color:teal;color:red;\">xxFj9E</span>HN\nrX<span style=\"background-color:teal;color:red;\">7=</span>W;<span style=\"background-color:teal;color:red;\">uEZii2</span>*x<span style=\"background-color:teal;color:red;\">os</span>Zu<span style=\"background-color:teal;color:red;\">DR</span>Bg\nvH<span style=\"background-color:teal;color:red;\">tk</span>D4<span style=\"background-color:teal;color:red;\">11dJ&gt;P</span>~3<span style=\"background-color:teal;color:red;\">=k</span>kB<span style=\"background-color:teal;color:red;\">4w</span>U^\n,6oFB+gD3f_%gJua{50=HD\nrq5?T0lL06Vg-[0G:,sum)</pre>\n<pre class=\"EXAMPLE EXAMPLE-f1 \" style=\"position:absolute;\">\nlH&lt;^&amp;?@^Hbt^3v5]7gx9&lt;o\nhoB&lt;mg&gt;DGKGfM3woWh4%EC\nJs%6!J<span style=\"background-color:teal;color:red;\">G[m,C8</span>PVsrG7_ESx\ndCX&lt;E6<span style=\"background-color:teal;color:red;\">]xxP~H</span>k(([D*dGU?\n~!gdx`w[f0<span style=\"background-color:teal;color:red;\">z)</span>xK<span style=\"background-color:teal;color:red;\">bnj{rA</span>WS\n5V=hf%`00g<span style=\"background-color:teal;color:red;\">l2</span>-Y<span style=\"background-color:teal;color:red;\">lT;L*&amp;</span>U`\nqk[3_h<span style=\"background-color:teal;color:red;\">DMUNNI</span>n}<span style=\"background-color:teal;color:red;\">Y`</span>F&gt;<span style=\"background-color:teal;color:red;\">lh</span>@o\nHsHD0=<span style=\"background-color:teal;color:red;\">38OO.#</span>iC<span style=\"background-color:teal;color:red;\">A&amp;</span>~U<span style=\"background-color:teal;color:red;\">[{</span>kT\nv#x@h.OzV,<span style=\"background-color:teal;color:red;\">zL</span>I6<span style=\"background-color:teal;color:red;\">#5&amp;kv4</span>T-\n#t:Gic$*w-<span style=\"background-color:teal;color:red;\">i;</span>P&lt;<span style=\"background-color:teal;color:red;\">O!fIWd</span>C,\nzs<span style=\"background-color:teal;color:red;\">{l</span>k^<span style=\"background-color:teal;color:red;\">a3Ty8S</span>bY<span style=\"background-color:teal;color:red;\">gL</span>Ge<span style=\"background-color:teal;color:red;\">7P</span>dt\ni#<span style=\"background-color:teal;color:red;\">%5</span>(w<span style=\"background-color:teal;color:red;\">~u9+cd</span>lO<span style=\"background-color:teal;color:red;\">dO</span>!&amp;<span style=\"background-color:teal;color:red;\">Ms</span>}6\nOu=JC8Zn.T;pq98cx)ov&gt;~\naNvuQi.U6%7-Kf,uk{FG[J</pre>\n<pre class=\"EXAMPLE EXAMPLE-f2 \" style=\"position:absolute;\">\nJk%bTX:]aZ,D?jkB0?*I*O\nr~&amp;Em{~S%FVCv._]xdt5.,\nOEr=o[<span style=\"background-color:teal;color:red;\">s:ocqC</span>a;,h2a-d:q\nU6]~]f<span style=\"background-color:teal;color:red;\">X&gt;~%T%</span>(RAa$p`~n!\nYRq`hu=gRK<span style=\"background-color:teal;color:red;\">g.</span>!k<span style=\"background-color:teal;color:red;\">&gt;82v#[</span>^D\nC#F7Nq3U3!<span style=\"background-color:teal;color:red;\">yo</span>pF<span style=\"background-color:teal;color:red;\">gBSRoU</span>PB\n)p_U9F<span style=\"background-color:teal;color:red;\">hxi]W1</span>IE<span style=\"background-color:teal;color:red;\">)=</span>s$<span style=\"background-color:teal;color:red;\">&gt;d</span>h^\n8uhRPg<span style=\"background-color:teal;color:red;\">Ay2}ui</span>oT<span style=\"background-color:teal;color:red;\">M5</span>Rw<span style=\"background-color:teal;color:red;\">@&gt;</span>*5\nxgq#T=V5d}<span style=\"background-color:teal;color:red;\">8W</span>aQ<span style=\"background-color:teal;color:red;\">U+kez]</span>&gt;X\ned`pS6%DK9<span style=\"background-color:teal;color:red;\">N%</span> i<span style=\"background-color:teal;color:red;\">Qo7-[g</span>DO\n[Z<span style=\"background-color:teal;color:red;\">:j</span>GU<span style=\"background-color:teal;color:red;\">Y%L&amp;$7</span>4[<span style=\"background-color:teal;color:red;\">@Q</span>8;<span style=\"background-color:teal;color:red;\">Km</span>~E\ngQ<span style=\"background-color:teal;color:red;\">{i</span>kr<span style=\"background-color:teal;color:red;\">-5fyM&lt;</span>{n<span style=\"background-color:teal;color:red;\">y6</span>=]<span style=\"background-color:teal;color:red;\">r4</span>U$\niVG.vOv5uWkulYY#GT[&amp;Tm\nU0DC_D-@ml4[7sP7&amp;)C9Q&gt;</pre>\n</div>";
    #[test]
    fn test_example(){
        let apple = rs3a::load(EXAMPLE.to_string()).unwrap();
        assert_eq!(
            VALID_EXAMPLE,
            render(&apple, None, None, "PREFIX", "EXAMPLE"),
        );
    }
    #[test]
    fn test_example_independent(){
        let apple = rs3a::load(EXAMPLE.to_string()).unwrap();
        assert_eq!(
            VALID_EXAMPLE_INDEPENDENT,
            render(&apple, None, Some(collection!{
                "independent".to_string() => "true".to_string(),
            }), "PREFIX", "EXAMPLE"),
        );
    }
    const FLASK: &str = r###"
@ This work is licensed under the Creative Commons Attribution 4.0 International License. To view a copy of this license, visit
@ http://creativecommons.org/licenses/by/4.0/.
title DNA
author DomesticMoth
width 9
height 14
loop true
colors fg

g-------C51118888c
 c-----G 0611888d0
   t-A   000e89000
    T    0000e0000
   C-g   000c85000
 G-----c 0d8881160
A-------t988881117
g-------C51118888c
 c-----G 0611888d0
   t-A   000789000
    T    0000e0000
   C-g   000c85000
 G-----c 0d8881160
A-------t988881117

 c-----G 0611888d0
   t-A   000e89000
    T    0000e0000
   C-g   000c85000
 G-----c 0d8881160
A-------t988881117
g-------C51118888c
 c-----G 0611888d0
   t-A   000789000
    T    0000e0000
   C-g   000c85000
 G-----c 0d8881160
A-------t988881117
g-------C51118888c

   t-A   000e89000
    T    0000e0000
   C-g   000c85000
 G-----c 0d8881160
A-------t988881117
g-------C51118888c
 c-----G 0611888d0
   t-A   000789000
    T    0000e0000
   C-g   000c85000
 G-----c 0d8881160
A-------t988881117
g-------C51118888c
 c-----G 0611888d0

    T    0000e0000
   C-g   000c85000
 G-----c 0d8881160
A-------t988881117
g-------C51118888c
 c-----G 0611888d0
   t-A   000789000
    T    0000e0000
   C-g   000c85000
 G-----c 0d8881160
A-------t988881117
g-------C51118888c
 c-----G 0611888d0
   t-A   000e89000

   C-g   000c85000
 G-----c 0d8881160
A-------t988881117
g-------C51118888c
 c-----G 0611888d0
   t-A   000789000
    T    0000e0000
   C-g   000c85000
 G-----c 0d8881160
A-------t988881117
g-------C51118888c
 c-----G 0611888d0
   t-A   000e89000
    T    0000e0000

 G-----c 0d8881160
A-------t988881117
g-------C51118888c
 c-----G 0611888d0
   t-A   000789000
    T    0000e0000
   C-g   000c85000
 G-----c 0d8881160
A-------t988881117
g-------C51118888c
 c-----G 0611888d0
   t-A   000e89000
    T    0000e0000
   C-g   000c85000

A-------t988881117
g-------C51118888c
 c-----G 0611888d0
   t-A   000789000
    T    0000e0000
   C-g   000c85000
 G-----c 0d8881160
A-------t988881117
g-------C51118888c
 c-----G 0611888d0
   t-A   000e89000
    T    0000e0000
   C-g   000c85000
 G-----c 0d8881160

g-------C51118888c
 c-----G 0611888d0
   t-A   000789000
    T    0000e0000
   C-g   000c85000
 G-----c 0d8881160
A-------t988881117
g-------C51118888c
 c-----G 0611888d0
   t-A   000e89000
    T    0000e0000
   C-g   000c85000
 G-----c 0d8881160
A-------t988881117
    "###;
    const VALID_FLASK: &str = "<div class=\"PREFIXfg-gray \" style=\"background-color:none;display:inline-block;position:relative;\" title=\"DNA\" author=\"DomesticMoth\">\n<style type=\"text/css\">.FLASK{visibility:hidden;display:inline-block;top:0;left:0;margin:0;}.FLASK-f0{animation:FLASK-f0a 0.4s infinite;}@keyframes FLASK-f0a{0%{visibility:hidden;}6.25%{visibility:visible;}12.5%{visibility:hidden;}}.FLASK-f1{animation:FLASK-f1a 0.4s infinite;}@keyframes FLASK-f1a{12.5%{visibility:hidden;}18.75%{visibility:visible;}25%{visibility:hidden;}}.FLASK-f2{animation:FLASK-f2a 0.4s infinite;}@keyframes FLASK-f2a{25%{visibility:hidden;}31.25%{visibility:visible;}37.5%{visibility:hidden;}}.FLASK-f3{animation:FLASK-f3a 0.4s infinite;}@keyframes FLASK-f3a{37.5%{visibility:hidden;}43.75%{visibility:visible;}50%{visibility:hidden;}}.FLASK-f4{animation:FLASK-f4a 0.4s infinite;}@keyframes FLASK-f4a{50%{visibility:hidden;}56.25%{visibility:visible;}62.5%{visibility:hidden;}}.FLASK-f5{animation:FLASK-f5a 0.4s infinite;}@keyframes FLASK-f5a{62.5%{visibility:hidden;}68.75%{visibility:visible;}75%{visibility:hidden;}}.FLASK-f6{animation:FLASK-f6a 0.4s infinite;}@keyframes FLASK-f6a{75%{visibility:hidden;}81.25%{visibility:visible;}87.5%{visibility:hidden;}}.FLASK-f7{animation:FLASK-f7a 0.4s infinite;}@keyframes FLASK-f7a{87.5%{visibility:hidden;}93.75%{visibility:visible;}100%{visibility:hidden;}}</style>\n<pre class=\"FLASK FLASK-f0 \">\n<span class=\"PREFIXfg-magenta \" style=\"background-color:none;\">g</span><span class=\"PREFIXfg-blue \" style=\"background-color:none;\">---</span>----<span class=\"PREFIXfg-bred \" style=\"background-color:none;\">C\n </span><span class=\"PREFIXfg-yellow \" style=\"background-color:none;\">c</span><span class=\"PREFIXfg-blue \" style=\"background-color:none;\">--</span>---<span class=\"PREFIXfg-bmagenta \" style=\"background-color:none;\">G \n   </span><span class=\"PREFIXfg-byellow \" style=\"background-color:none;\">t</span>-<span class=\"PREFIXfg-bblue \" style=\"background-color:none;\">A   \n    </span><span class=\"PREFIXfg-byellow \" style=\"background-color:none;\">T    \n   </span><span class=\"PREFIXfg-bred \" style=\"background-color:none;\">C</span>-<span class=\"PREFIXfg-magenta \" style=\"background-color:none;\">g   \n </span><span class=\"PREFIXfg-bmagenta \" style=\"background-color:none;\">G</span>---<span class=\"PREFIXfg-blue \" style=\"background-color:none;\">--</span><span class=\"PREFIXfg-yellow \" style=\"background-color:none;\">c \n</span><span class=\"PREFIXfg-bblue \" style=\"background-color:none;\">A</span>----<span class=\"PREFIXfg-blue \" style=\"background-color:none;\">---</span><span class=\"PREFIXfg-white \" style=\"background-color:none;\">t\n</span><span class=\"PREFIXfg-magenta \" style=\"background-color:none;\">g</span><span class=\"PREFIXfg-blue \" style=\"background-color:none;\">---</span>----<span class=\"PREFIXfg-bred \" style=\"background-color:none;\">C\n </span><span class=\"PREFIXfg-yellow \" style=\"background-color:none;\">c</span><span class=\"PREFIXfg-blue \" style=\"background-color:none;\">--</span>---<span class=\"PREFIXfg-bmagenta \" style=\"background-color:none;\">G \n   </span><span class=\"PREFIXfg-white \" style=\"background-color:none;\">t</span>-<span class=\"PREFIXfg-bblue \" style=\"background-color:none;\">A   \n    </span><span class=\"PREFIXfg-byellow \" style=\"background-color:none;\">T    \n   </span><span class=\"PREFIXfg-bred \" style=\"background-color:none;\">C</span>-<span class=\"PREFIXfg-magenta \" style=\"background-color:none;\">g   \n </span><span class=\"PREFIXfg-bmagenta \" style=\"background-color:none;\">G</span>---<span class=\"PREFIXfg-blue \" style=\"background-color:none;\">--</span><span class=\"PREFIXfg-yellow \" style=\"background-color:none;\">c \n</span><span class=\"PREFIXfg-bblue \" style=\"background-color:none;\">A</span>----<span class=\"PREFIXfg-blue \" style=\"background-color:none;\">---</span><span class=\"PREFIXfg-white \" style=\"background-color:none;\">t</pre>\n<pre class=\"FLASK FLASK-f1 \" style=\"position:absolute;\">\n <span class=\"PREFIXfg-yellow \" style=\"background-color:none;\">c</span><span class=\"PREFIXfg-blue \" style=\"background-color:none;\">--</span>---<span class=\"PREFIXfg-bmagenta \" style=\"background-color:none;\">G \n   </span><span class=\"PREFIXfg-byellow \" style=\"background-color:none;\">t</span>-<span class=\"PREFIXfg-bblue \" style=\"background-color:none;\">A   \n    </span><span class=\"PREFIXfg-byellow \" style=\"background-color:none;\">T    \n   </span><span class=\"PREFIXfg-bred \" style=\"background-color:none;\">C</span>-<span class=\"PREFIXfg-magenta \" style=\"background-color:none;\">g   \n </span><span class=\"PREFIXfg-bmagenta \" style=\"background-color:none;\">G</span>---<span class=\"PREFIXfg-blue \" style=\"background-color:none;\">--</span><span class=\"PREFIXfg-yellow \" style=\"background-color:none;\">c \n</span><span class=\"PREFIXfg-bblue \" style=\"background-color:none;\">A</span>----<span class=\"PREFIXfg-blue \" style=\"background-color:none;\">---</span><span class=\"PREFIXfg-white \" style=\"background-color:none;\">t\n</span><span class=\"PREFIXfg-magenta \" style=\"background-color:none;\">g</span><span class=\"PREFIXfg-blue \" style=\"background-color:none;\">---</span>----<span class=\"PREFIXfg-bred \" style=\"background-color:none;\">C\n </span><span class=\"PREFIXfg-yellow \" style=\"background-color:none;\">c</span><span class=\"PREFIXfg-blue \" style=\"background-color:none;\">--</span>---<span class=\"PREFIXfg-bmagenta \" style=\"background-color:none;\">G \n   </span><span class=\"PREFIXfg-white \" style=\"background-color:none;\">t</span>-<span class=\"PREFIXfg-bblue \" style=\"background-color:none;\">A   \n    </span><span class=\"PREFIXfg-byellow \" style=\"background-color:none;\">T    \n   </span><span class=\"PREFIXfg-bred \" style=\"background-color:none;\">C</span>-<span class=\"PREFIXfg-magenta \" style=\"background-color:none;\">g   \n </span><span class=\"PREFIXfg-bmagenta \" style=\"background-color:none;\">G</span>---<span class=\"PREFIXfg-blue \" style=\"background-color:none;\">--</span><span class=\"PREFIXfg-yellow \" style=\"background-color:none;\">c \n</span><span class=\"PREFIXfg-bblue \" style=\"background-color:none;\">A</span>----<span class=\"PREFIXfg-blue \" style=\"background-color:none;\">---</span><span class=\"PREFIXfg-white \" style=\"background-color:none;\">t\n</span><span class=\"PREFIXfg-magenta \" style=\"background-color:none;\">g</span><span class=\"PREFIXfg-blue \" style=\"background-color:none;\">---</span>----<span class=\"PREFIXfg-bred \" style=\"background-color:none;\">C</pre>\n<pre class=\"FLASK FLASK-f2 \" style=\"position:absolute;\">\n   <span class=\"PREFIXfg-byellow \" style=\"background-color:none;\">t</span>-<span class=\"PREFIXfg-bblue \" style=\"background-color:none;\">A   \n    </span><span class=\"PREFIXfg-byellow \" style=\"background-color:none;\">T    \n   </span><span class=\"PREFIXfg-bred \" style=\"background-color:none;\">C</span>-<span class=\"PREFIXfg-magenta \" style=\"background-color:none;\">g   \n </span><span class=\"PREFIXfg-bmagenta \" style=\"background-color:none;\">G</span>---<span class=\"PREFIXfg-blue \" style=\"background-color:none;\">--</span><span class=\"PREFIXfg-yellow \" style=\"background-color:none;\">c \n</span><span class=\"PREFIXfg-bblue \" style=\"background-color:none;\">A</span>----<span class=\"PREFIXfg-blue \" style=\"background-color:none;\">---</span><span class=\"PREFIXfg-white \" style=\"background-color:none;\">t\n</span><span class=\"PREFIXfg-magenta \" style=\"background-color:none;\">g</span><span class=\"PREFIXfg-blue \" style=\"background-color:none;\">---</span>----<span class=\"PREFIXfg-bred \" style=\"background-color:none;\">C\n </span><span class=\"PREFIXfg-yellow \" style=\"background-color:none;\">c</span><span class=\"PREFIXfg-blue \" style=\"background-color:none;\">--</span>---<span class=\"PREFIXfg-bmagenta \" style=\"background-color:none;\">G \n   </span><span class=\"PREFIXfg-white \" style=\"background-color:none;\">t</span>-<span class=\"PREFIXfg-bblue \" style=\"background-color:none;\">A   \n    </span><span class=\"PREFIXfg-byellow \" style=\"background-color:none;\">T    \n   </span><span class=\"PREFIXfg-bred \" style=\"background-color:none;\">C</span>-<span class=\"PREFIXfg-magenta \" style=\"background-color:none;\">g   \n </span><span class=\"PREFIXfg-bmagenta \" style=\"background-color:none;\">G</span>---<span class=\"PREFIXfg-blue \" style=\"background-color:none;\">--</span><span class=\"PREFIXfg-yellow \" style=\"background-color:none;\">c \n</span><span class=\"PREFIXfg-bblue \" style=\"background-color:none;\">A</span>----<span class=\"PREFIXfg-blue \" style=\"background-color:none;\">---</span><span class=\"PREFIXfg-white \" style=\"background-color:none;\">t\n</span><span class=\"PREFIXfg-magenta \" style=\"background-color:none;\">g</span><span class=\"PREFIXfg-blue \" style=\"background-color:none;\">---</span>----<span class=\"PREFIXfg-bred \" style=\"background-color:none;\">C\n </span><span class=\"PREFIXfg-yellow \" style=\"background-color:none;\">c</span><span class=\"PREFIXfg-blue \" style=\"background-color:none;\">--</span>---<span class=\"PREFIXfg-bmagenta \" style=\"background-color:none;\">G </pre>\n<pre class=\"FLASK FLASK-f3 \" style=\"position:absolute;\">\n    <span class=\"PREFIXfg-byellow \" style=\"background-color:none;\">T    \n   </span><span class=\"PREFIXfg-bred \" style=\"background-color:none;\">C</span>-<span class=\"PREFIXfg-magenta \" style=\"background-color:none;\">g   \n </span><span class=\"PREFIXfg-bmagenta \" style=\"background-color:none;\">G</span>---<span class=\"PREFIXfg-blue \" style=\"background-color:none;\">--</span><span class=\"PREFIXfg-yellow \" style=\"background-color:none;\">c \n</span><span class=\"PREFIXfg-bblue \" style=\"background-color:none;\">A</span>----<span class=\"PREFIXfg-blue \" style=\"background-color:none;\">---</span><span class=\"PREFIXfg-white \" style=\"background-color:none;\">t\n</span><span class=\"PREFIXfg-magenta \" style=\"background-color:none;\">g</span><span class=\"PREFIXfg-blue \" style=\"background-color:none;\">---</span>----<span class=\"PREFIXfg-bred \" style=\"background-color:none;\">C\n </span><span class=\"PREFIXfg-yellow \" style=\"background-color:none;\">c</span><span class=\"PREFIXfg-blue \" style=\"background-color:none;\">--</span>---<span class=\"PREFIXfg-bmagenta \" style=\"background-color:none;\">G \n   </span><span class=\"PREFIXfg-white \" style=\"background-color:none;\">t</span>-<span class=\"PREFIXfg-bblue \" style=\"background-color:none;\">A   \n    </span><span class=\"PREFIXfg-byellow \" style=\"background-color:none;\">T    \n   </span><span class=\"PREFIXfg-bred \" style=\"background-color:none;\">C</span>-<span class=\"PREFIXfg-magenta \" style=\"background-color:none;\">g   \n </span><span class=\"PREFIXfg-bmagenta \" style=\"background-color:none;\">G</span>---<span class=\"PREFIXfg-blue \" style=\"background-color:none;\">--</span><span class=\"PREFIXfg-yellow \" style=\"background-color:none;\">c \n</span><span class=\"PREFIXfg-bblue \" style=\"background-color:none;\">A</span>----<span class=\"PREFIXfg-blue \" style=\"background-color:none;\">---</span><span class=\"PREFIXfg-white \" style=\"background-color:none;\">t\n</span><span class=\"PREFIXfg-magenta \" style=\"background-color:none;\">g</span><span class=\"PREFIXfg-blue \" style=\"background-color:none;\">---</span>----<span class=\"PREFIXfg-bred \" style=\"background-color:none;\">C\n </span><span class=\"PREFIXfg-yellow \" style=\"background-color:none;\">c</span><span class=\"PREFIXfg-blue \" style=\"background-color:none;\">--</span>---<span class=\"PREFIXfg-bmagenta \" style=\"background-color:none;\">G \n   </span><span class=\"PREFIXfg-byellow \" style=\"background-color:none;\">t</span>-<span class=\"PREFIXfg-bblue \" style=\"background-color:none;\">A   </pre>\n<pre class=\"FLASK FLASK-f4 \" style=\"position:absolute;\">\n   <span class=\"PREFIXfg-bred \" style=\"background-color:none;\">C</span>-<span class=\"PREFIXfg-magenta \" style=\"background-color:none;\">g   \n </span><span class=\"PREFIXfg-bmagenta \" style=\"background-color:none;\">G</span>---<span class=\"PREFIXfg-blue \" style=\"background-color:none;\">--</span><span class=\"PREFIXfg-yellow \" style=\"background-color:none;\">c \n</span><span class=\"PREFIXfg-bblue \" style=\"background-color:none;\">A</span>----<span class=\"PREFIXfg-blue \" style=\"background-color:none;\">---</span><span class=\"PREFIXfg-white \" style=\"background-color:none;\">t\n</span><span class=\"PREFIXfg-magenta \" style=\"background-color:none;\">g</span><span class=\"PREFIXfg-blue \" style=\"background-color:none;\">---</span>----<span class=\"PREFIXfg-bred \" style=\"background-color:none;\">C\n </span><span class=\"PREFIXfg-yellow \" style=\"background-color:none;\">c</span><span class=\"PREFIXfg-blue \" style=\"background-color:none;\">--</span>---<span class=\"PREFIXfg-bmagenta \" style=\"background-color:none;\">G \n   </span><span class=\"PREFIXfg-white \" style=\"background-color:none;\">t</span>-<span class=\"PREFIXfg-bblue \" style=\"background-color:none;\">A   \n    </span><span class=\"PREFIXfg-byellow \" style=\"background-color:none;\">T    \n   </span><span class=\"PREFIXfg-bred \" style=\"background-color:none;\">C</span>-<span class=\"PREFIXfg-magenta \" style=\"background-color:none;\">g   \n </span><span class=\"PREFIXfg-bmagenta \" style=\"background-color:none;\">G</span>---<span class=\"PREFIXfg-blue \" style=\"background-color:none;\">--</span><span class=\"PREFIXfg-yellow \" style=\"background-color:none;\">c \n</span><span class=\"PREFIXfg-bblue \" style=\"background-color:none;\">A</span>----<span class=\"PREFIXfg-blue \" style=\"background-color:none;\">---</span><span class=\"PREFIXfg-white \" style=\"background-color:none;\">t\n</span><span class=\"PREFIXfg-magenta \" style=\"background-color:none;\">g</span><span class=\"PREFIXfg-blue \" style=\"background-color:none;\">---</span>----<span class=\"PREFIXfg-bred \" style=\"background-color:none;\">C\n </span><span class=\"PREFIXfg-yellow \" style=\"background-color:none;\">c</span><span class=\"PREFIXfg-blue \" style=\"background-color:none;\">--</span>---<span class=\"PREFIXfg-bmagenta \" style=\"background-color:none;\">G \n   </span><span class=\"PREFIXfg-byellow \" style=\"background-color:none;\">t</span>-<span class=\"PREFIXfg-bblue \" style=\"background-color:none;\">A   \n    </span><span class=\"PREFIXfg-byellow \" style=\"background-color:none;\">T    </pre>\n<pre class=\"FLASK FLASK-f5 \" style=\"position:absolute;\">\n <span class=\"PREFIXfg-bmagenta \" style=\"background-color:none;\">G</span>---<span class=\"PREFIXfg-blue \" style=\"background-color:none;\">--</span><span class=\"PREFIXfg-yellow \" style=\"background-color:none;\">c \n</span><span class=\"PREFIXfg-bblue \" style=\"background-color:none;\">A</span>----<span class=\"PREFIXfg-blue \" style=\"background-color:none;\">---</span><span class=\"PREFIXfg-white \" style=\"background-color:none;\">t\n</span><span class=\"PREFIXfg-magenta \" style=\"background-color:none;\">g</span><span class=\"PREFIXfg-blue \" style=\"background-color:none;\">---</span>----<span class=\"PREFIXfg-bred \" style=\"background-color:none;\">C\n </span><span class=\"PREFIXfg-yellow \" style=\"background-color:none;\">c</span><span class=\"PREFIXfg-blue \" style=\"background-color:none;\">--</span>---<span class=\"PREFIXfg-bmagenta \" style=\"background-color:none;\">G \n   </span><span class=\"PREFIXfg-white \" style=\"background-color:none;\">t</span>-<span class=\"PREFIXfg-bblue \" style=\"background-color:none;\">A   \n    </span><span class=\"PREFIXfg-byellow \" style=\"background-color:none;\">T    \n   </span><span class=\"PREFIXfg-bred \" style=\"background-color:none;\">C</span>-<span class=\"PREFIXfg-magenta \" style=\"background-color:none;\">g   \n </span><span class=\"PREFIXfg-bmagenta \" style=\"background-color:none;\">G</span>---<span class=\"PREFIXfg-blue \" style=\"background-color:none;\">--</span><span class=\"PREFIXfg-yellow \" style=\"background-color:none;\">c \n</span><span class=\"PREFIXfg-bblue \" style=\"background-color:none;\">A</span>----<span class=\"PREFIXfg-blue \" style=\"background-color:none;\">---</span><span class=\"PREFIXfg-white \" style=\"background-color:none;\">t\n</span><span class=\"PREFIXfg-magenta \" style=\"background-color:none;\">g</span><span class=\"PREFIXfg-blue \" style=\"background-color:none;\">---</span>----<span class=\"PREFIXfg-bred \" style=\"background-color:none;\">C\n </span><span class=\"PREFIXfg-yellow \" style=\"background-color:none;\">c</span><span class=\"PREFIXfg-blue \" style=\"background-color:none;\">--</span>---<span class=\"PREFIXfg-bmagenta \" style=\"background-color:none;\">G \n   </span><span class=\"PREFIXfg-byellow \" style=\"background-color:none;\">t</span>-<span class=\"PREFIXfg-bblue \" style=\"background-color:none;\">A   \n    </span><span class=\"PREFIXfg-byellow \" style=\"background-color:none;\">T    \n   </span><span class=\"PREFIXfg-bred \" style=\"background-color:none;\">C</span>-<span class=\"PREFIXfg-magenta \" style=\"background-color:none;\">g   </pre>\n<pre class=\"FLASK FLASK-f6 \" style=\"position:absolute;\">\n<span class=\"PREFIXfg-bblue \" style=\"background-color:none;\">A</span>----<span class=\"PREFIXfg-blue \" style=\"background-color:none;\">---</span><span class=\"PREFIXfg-white \" style=\"background-color:none;\">t\n</span><span class=\"PREFIXfg-magenta \" style=\"background-color:none;\">g</span><span class=\"PREFIXfg-blue \" style=\"background-color:none;\">---</span>----<span class=\"PREFIXfg-bred \" style=\"background-color:none;\">C\n </span><span class=\"PREFIXfg-yellow \" style=\"background-color:none;\">c</span><span class=\"PREFIXfg-blue \" style=\"background-color:none;\">--</span>---<span class=\"PREFIXfg-bmagenta \" style=\"background-color:none;\">G \n   </span><span class=\"PREFIXfg-white \" style=\"background-color:none;\">t</span>-<span class=\"PREFIXfg-bblue \" style=\"background-color:none;\">A   \n    </span><span class=\"PREFIXfg-byellow \" style=\"background-color:none;\">T    \n   </span><span class=\"PREFIXfg-bred \" style=\"background-color:none;\">C</span>-<span class=\"PREFIXfg-magenta \" style=\"background-color:none;\">g   \n </span><span class=\"PREFIXfg-bmagenta \" style=\"background-color:none;\">G</span>---<span class=\"PREFIXfg-blue \" style=\"background-color:none;\">--</span><span class=\"PREFIXfg-yellow \" style=\"background-color:none;\">c \n</span><span class=\"PREFIXfg-bblue \" style=\"background-color:none;\">A</span>----<span class=\"PREFIXfg-blue \" style=\"background-color:none;\">---</span><span class=\"PREFIXfg-white \" style=\"background-color:none;\">t\n</span><span class=\"PREFIXfg-magenta \" style=\"background-color:none;\">g</span><span class=\"PREFIXfg-blue \" style=\"background-color:none;\">---</span>----<span class=\"PREFIXfg-bred \" style=\"background-color:none;\">C\n </span><span class=\"PREFIXfg-yellow \" style=\"background-color:none;\">c</span><span class=\"PREFIXfg-blue \" style=\"background-color:none;\">--</span>---<span class=\"PREFIXfg-bmagenta \" style=\"background-color:none;\">G \n   </span><span class=\"PREFIXfg-byellow \" style=\"background-color:none;\">t</span>-<span class=\"PREFIXfg-bblue \" style=\"background-color:none;\">A   \n    </span><span class=\"PREFIXfg-byellow \" style=\"background-color:none;\">T    \n   </span><span class=\"PREFIXfg-bred \" style=\"background-color:none;\">C</span>-<span class=\"PREFIXfg-magenta \" style=\"background-color:none;\">g   \n </span><span class=\"PREFIXfg-bmagenta \" style=\"background-color:none;\">G</span>---<span class=\"PREFIXfg-blue \" style=\"background-color:none;\">--</span><span class=\"PREFIXfg-yellow \" style=\"background-color:none;\">c </pre>\n<pre class=\"FLASK FLASK-f7 \" style=\"position:absolute;\">\n<span class=\"PREFIXfg-magenta \" style=\"background-color:none;\">g</span><span class=\"PREFIXfg-blue \" style=\"background-color:none;\">---</span>----<span class=\"PREFIXfg-bred \" style=\"background-color:none;\">C\n </span><span class=\"PREFIXfg-yellow \" style=\"background-color:none;\">c</span><span class=\"PREFIXfg-blue \" style=\"background-color:none;\">--</span>---<span class=\"PREFIXfg-bmagenta \" style=\"background-color:none;\">G \n   </span><span class=\"PREFIXfg-white \" style=\"background-color:none;\">t</span>-<span class=\"PREFIXfg-bblue \" style=\"background-color:none;\">A   \n    </span><span class=\"PREFIXfg-byellow \" style=\"background-color:none;\">T    \n   </span><span class=\"PREFIXfg-bred \" style=\"background-color:none;\">C</span>-<span class=\"PREFIXfg-magenta \" style=\"background-color:none;\">g   \n </span><span class=\"PREFIXfg-bmagenta \" style=\"background-color:none;\">G</span>---<span class=\"PREFIXfg-blue \" style=\"background-color:none;\">--</span><span class=\"PREFIXfg-yellow \" style=\"background-color:none;\">c \n</span><span class=\"PREFIXfg-bblue \" style=\"background-color:none;\">A</span>----<span class=\"PREFIXfg-blue \" style=\"background-color:none;\">---</span><span class=\"PREFIXfg-white \" style=\"background-color:none;\">t\n</span><span class=\"PREFIXfg-magenta \" style=\"background-color:none;\">g</span><span class=\"PREFIXfg-blue \" style=\"background-color:none;\">---</span>----<span class=\"PREFIXfg-bred \" style=\"background-color:none;\">C\n </span><span class=\"PREFIXfg-yellow \" style=\"background-color:none;\">c</span><span class=\"PREFIXfg-blue \" style=\"background-color:none;\">--</span>---<span class=\"PREFIXfg-bmagenta \" style=\"background-color:none;\">G \n   </span><span class=\"PREFIXfg-byellow \" style=\"background-color:none;\">t</span>-<span class=\"PREFIXfg-bblue \" style=\"background-color:none;\">A   \n    </span><span class=\"PREFIXfg-byellow \" style=\"background-color:none;\">T    \n   </span><span class=\"PREFIXfg-bred \" style=\"background-color:none;\">C</span>-<span class=\"PREFIXfg-magenta \" style=\"background-color:none;\">g   \n </span><span class=\"PREFIXfg-bmagenta \" style=\"background-color:none;\">G</span>---<span class=\"PREFIXfg-blue \" style=\"background-color:none;\">--</span><span class=\"PREFIXfg-yellow \" style=\"background-color:none;\">c \n</span><span class=\"PREFIXfg-bblue \" style=\"background-color:none;\">A</span>----<span class=\"PREFIXfg-blue \" style=\"background-color:none;\">---</span><span class=\"PREFIXfg-white \" style=\"background-color:none;\">t</pre>\n</div>";
    const VALID_FLASK_INDEPENDENT: &str = "<div class=\"\" style=\"background-color:none;color:gray;display:inline-block;independent:true;position:relative;\" title=\"DNA\" author=\"DomesticMoth\">\n<style type=\"text/css\">.FLASK{visibility:hidden;display:inline-block;top:0;left:0;margin:0;}.FLASK-f0{animation:FLASK-f0a 0.4s infinite;}@keyframes FLASK-f0a{0%{visibility:hidden;}6.25%{visibility:visible;}12.5%{visibility:hidden;}}.FLASK-f1{animation:FLASK-f1a 0.4s infinite;}@keyframes FLASK-f1a{12.5%{visibility:hidden;}18.75%{visibility:visible;}25%{visibility:hidden;}}.FLASK-f2{animation:FLASK-f2a 0.4s infinite;}@keyframes FLASK-f2a{25%{visibility:hidden;}31.25%{visibility:visible;}37.5%{visibility:hidden;}}.FLASK-f3{animation:FLASK-f3a 0.4s infinite;}@keyframes FLASK-f3a{37.5%{visibility:hidden;}43.75%{visibility:visible;}50%{visibility:hidden;}}.FLASK-f4{animation:FLASK-f4a 0.4s infinite;}@keyframes FLASK-f4a{50%{visibility:hidden;}56.25%{visibility:visible;}62.5%{visibility:hidden;}}.FLASK-f5{animation:FLASK-f5a 0.4s infinite;}@keyframes FLASK-f5a{62.5%{visibility:hidden;}68.75%{visibility:visible;}75%{visibility:hidden;}}.FLASK-f6{animation:FLASK-f6a 0.4s infinite;}@keyframes FLASK-f6a{75%{visibility:hidden;}81.25%{visibility:visible;}87.5%{visibility:hidden;}}.FLASK-f7{animation:FLASK-f7a 0.4s infinite;}@keyframes FLASK-f7a{87.5%{visibility:hidden;}93.75%{visibility:visible;}100%{visibility:hidden;}}</style>\n<pre class=\"FLASK FLASK-f0 \">\n<span style=\"background-color:none;color:purple;\">g</span><span style=\"background-color:none;color:navy;\">---</span>----<span style=\"background-color:none;color:red;\">C\n </span><span style=\"background-color:none;color:olive;\">c</span><span style=\"background-color:none;color:navy;\">--</span>---<span style=\"background-color:none;color:fuchsia;\">G \n   </span><span style=\"background-color:none;color:yellow;\">t</span>-<span style=\"background-color:none;color:blue;\">A   \n    </span><span style=\"background-color:none;color:yellow;\">T    \n   </span><span style=\"background-color:none;color:red;\">C</span>-<span style=\"background-color:none;color:purple;\">g   \n </span><span style=\"background-color:none;color:fuchsia;\">G</span>---<span style=\"background-color:none;color:navy;\">--</span><span style=\"background-color:none;color:olive;\">c \n</span><span style=\"background-color:none;color:blue;\">A</span>----<span style=\"background-color:none;color:navy;\">---</span><span style=\"background-color:none;color:silver;\">t\n</span><span style=\"background-color:none;color:purple;\">g</span><span style=\"background-color:none;color:navy;\">---</span>----<span style=\"background-color:none;color:red;\">C\n </span><span style=\"background-color:none;color:olive;\">c</span><span style=\"background-color:none;color:navy;\">--</span>---<span style=\"background-color:none;color:fuchsia;\">G \n   </span><span style=\"background-color:none;color:silver;\">t</span>-<span style=\"background-color:none;color:blue;\">A   \n    </span><span style=\"background-color:none;color:yellow;\">T    \n   </span><span style=\"background-color:none;color:red;\">C</span>-<span style=\"background-color:none;color:purple;\">g   \n </span><span style=\"background-color:none;color:fuchsia;\">G</span>---<span style=\"background-color:none;color:navy;\">--</span><span style=\"background-color:none;color:olive;\">c \n</span><span style=\"background-color:none;color:blue;\">A</span>----<span style=\"background-color:none;color:navy;\">---</span><span style=\"background-color:none;color:silver;\">t</pre>\n<pre class=\"FLASK FLASK-f1 \" style=\"position:absolute;\">\n <span style=\"background-color:none;color:olive;\">c</span><span style=\"background-color:none;color:navy;\">--</span>---<span style=\"background-color:none;color:fuchsia;\">G \n   </span><span style=\"background-color:none;color:yellow;\">t</span>-<span style=\"background-color:none;color:blue;\">A   \n    </span><span style=\"background-color:none;color:yellow;\">T    \n   </span><span style=\"background-color:none;color:red;\">C</span>-<span style=\"background-color:none;color:purple;\">g   \n </span><span style=\"background-color:none;color:fuchsia;\">G</span>---<span style=\"background-color:none;color:navy;\">--</span><span style=\"background-color:none;color:olive;\">c \n</span><span style=\"background-color:none;color:blue;\">A</span>----<span style=\"background-color:none;color:navy;\">---</span><span style=\"background-color:none;color:silver;\">t\n</span><span style=\"background-color:none;color:purple;\">g</span><span style=\"background-color:none;color:navy;\">---</span>----<span style=\"background-color:none;color:red;\">C\n </span><span style=\"background-color:none;color:olive;\">c</span><span style=\"background-color:none;color:navy;\">--</span>---<span style=\"background-color:none;color:fuchsia;\">G \n   </span><span style=\"background-color:none;color:silver;\">t</span>-<span style=\"background-color:none;color:blue;\">A   \n    </span><span style=\"background-color:none;color:yellow;\">T    \n   </span><span style=\"background-color:none;color:red;\">C</span>-<span style=\"background-color:none;color:purple;\">g   \n </span><span style=\"background-color:none;color:fuchsia;\">G</span>---<span style=\"background-color:none;color:navy;\">--</span><span style=\"background-color:none;color:olive;\">c \n</span><span style=\"background-color:none;color:blue;\">A</span>----<span style=\"background-color:none;color:navy;\">---</span><span style=\"background-color:none;color:silver;\">t\n</span><span style=\"background-color:none;color:purple;\">g</span><span style=\"background-color:none;color:navy;\">---</span>----<span style=\"background-color:none;color:red;\">C</pre>\n<pre class=\"FLASK FLASK-f2 \" style=\"position:absolute;\">\n   <span style=\"background-color:none;color:yellow;\">t</span>-<span style=\"background-color:none;color:blue;\">A   \n    </span><span style=\"background-color:none;color:yellow;\">T    \n   </span><span style=\"background-color:none;color:red;\">C</span>-<span style=\"background-color:none;color:purple;\">g   \n </span><span style=\"background-color:none;color:fuchsia;\">G</span>---<span style=\"background-color:none;color:navy;\">--</span><span style=\"background-color:none;color:olive;\">c \n</span><span style=\"background-color:none;color:blue;\">A</span>----<span style=\"background-color:none;color:navy;\">---</span><span style=\"background-color:none;color:silver;\">t\n</span><span style=\"background-color:none;color:purple;\">g</span><span style=\"background-color:none;color:navy;\">---</span>----<span style=\"background-color:none;color:red;\">C\n </span><span style=\"background-color:none;color:olive;\">c</span><span style=\"background-color:none;color:navy;\">--</span>---<span style=\"background-color:none;color:fuchsia;\">G \n   </span><span style=\"background-color:none;color:silver;\">t</span>-<span style=\"background-color:none;color:blue;\">A   \n    </span><span style=\"background-color:none;color:yellow;\">T    \n   </span><span style=\"background-color:none;color:red;\">C</span>-<span style=\"background-color:none;color:purple;\">g   \n </span><span style=\"background-color:none;color:fuchsia;\">G</span>---<span style=\"background-color:none;color:navy;\">--</span><span style=\"background-color:none;color:olive;\">c \n</span><span style=\"background-color:none;color:blue;\">A</span>----<span style=\"background-color:none;color:navy;\">---</span><span style=\"background-color:none;color:silver;\">t\n</span><span style=\"background-color:none;color:purple;\">g</span><span style=\"background-color:none;color:navy;\">---</span>----<span style=\"background-color:none;color:red;\">C\n </span><span style=\"background-color:none;color:olive;\">c</span><span style=\"background-color:none;color:navy;\">--</span>---<span style=\"background-color:none;color:fuchsia;\">G </pre>\n<pre class=\"FLASK FLASK-f3 \" style=\"position:absolute;\">\n    <span style=\"background-color:none;color:yellow;\">T    \n   </span><span style=\"background-color:none;color:red;\">C</span>-<span style=\"background-color:none;color:purple;\">g   \n </span><span style=\"background-color:none;color:fuchsia;\">G</span>---<span style=\"background-color:none;color:navy;\">--</span><span style=\"background-color:none;color:olive;\">c \n</span><span style=\"background-color:none;color:blue;\">A</span>----<span style=\"background-color:none;color:navy;\">---</span><span style=\"background-color:none;color:silver;\">t\n</span><span style=\"background-color:none;color:purple;\">g</span><span style=\"background-color:none;color:navy;\">---</span>----<span style=\"background-color:none;color:red;\">C\n </span><span style=\"background-color:none;color:olive;\">c</span><span style=\"background-color:none;color:navy;\">--</span>---<span style=\"background-color:none;color:fuchsia;\">G \n   </span><span style=\"background-color:none;color:silver;\">t</span>-<span style=\"background-color:none;color:blue;\">A   \n    </span><span style=\"background-color:none;color:yellow;\">T    \n   </span><span style=\"background-color:none;color:red;\">C</span>-<span style=\"background-color:none;color:purple;\">g   \n </span><span style=\"background-color:none;color:fuchsia;\">G</span>---<span style=\"background-color:none;color:navy;\">--</span><span style=\"background-color:none;color:olive;\">c \n</span><span style=\"background-color:none;color:blue;\">A</span>----<span style=\"background-color:none;color:navy;\">---</span><span style=\"background-color:none;color:silver;\">t\n</span><span style=\"background-color:none;color:purple;\">g</span><span style=\"background-color:none;color:navy;\">---</span>----<span style=\"background-color:none;color:red;\">C\n </span><span style=\"background-color:none;color:olive;\">c</span><span style=\"background-color:none;color:navy;\">--</span>---<span style=\"background-color:none;color:fuchsia;\">G \n   </span><span style=\"background-color:none;color:yellow;\">t</span>-<span style=\"background-color:none;color:blue;\">A   </pre>\n<pre class=\"FLASK FLASK-f4 \" style=\"position:absolute;\">\n   <span style=\"background-color:none;color:red;\">C</span>-<span style=\"background-color:none;color:purple;\">g   \n </span><span style=\"background-color:none;color:fuchsia;\">G</span>---<span style=\"background-color:none;color:navy;\">--</span><span style=\"background-color:none;color:olive;\">c \n</span><span style=\"background-color:none;color:blue;\">A</span>----<span style=\"background-color:none;color:navy;\">---</span><span style=\"background-color:none;color:silver;\">t\n</span><span style=\"background-color:none;color:purple;\">g</span><span style=\"background-color:none;color:navy;\">---</span>----<span style=\"background-color:none;color:red;\">C\n </span><span style=\"background-color:none;color:olive;\">c</span><span style=\"background-color:none;color:navy;\">--</span>---<span style=\"background-color:none;color:fuchsia;\">G \n   </span><span style=\"background-color:none;color:silver;\">t</span>-<span style=\"background-color:none;color:blue;\">A   \n    </span><span style=\"background-color:none;color:yellow;\">T    \n   </span><span style=\"background-color:none;color:red;\">C</span>-<span style=\"background-color:none;color:purple;\">g   \n </span><span style=\"background-color:none;color:fuchsia;\">G</span>---<span style=\"background-color:none;color:navy;\">--</span><span style=\"background-color:none;color:olive;\">c \n</span><span style=\"background-color:none;color:blue;\">A</span>----<span style=\"background-color:none;color:navy;\">---</span><span style=\"background-color:none;color:silver;\">t\n</span><span style=\"background-color:none;color:purple;\">g</span><span style=\"background-color:none;color:navy;\">---</span>----<span style=\"background-color:none;color:red;\">C\n </span><span style=\"background-color:none;color:olive;\">c</span><span style=\"background-color:none;color:navy;\">--</span>---<span style=\"background-color:none;color:fuchsia;\">G \n   </span><span style=\"background-color:none;color:yellow;\">t</span>-<span style=\"background-color:none;color:blue;\">A   \n    </span><span style=\"background-color:none;color:yellow;\">T    </pre>\n<pre class=\"FLASK FLASK-f5 \" style=\"position:absolute;\">\n <span style=\"background-color:none;color:fuchsia;\">G</span>---<span style=\"background-color:none;color:navy;\">--</span><span style=\"background-color:none;color:olive;\">c \n</span><span style=\"background-color:none;color:blue;\">A</span>----<span style=\"background-color:none;color:navy;\">---</span><span style=\"background-color:none;color:silver;\">t\n</span><span style=\"background-color:none;color:purple;\">g</span><span style=\"background-color:none;color:navy;\">---</span>----<span style=\"background-color:none;color:red;\">C\n </span><span style=\"background-color:none;color:olive;\">c</span><span style=\"background-color:none;color:navy;\">--</span>---<span style=\"background-color:none;color:fuchsia;\">G \n   </span><span style=\"background-color:none;color:silver;\">t</span>-<span style=\"background-color:none;color:blue;\">A   \n    </span><span style=\"background-color:none;color:yellow;\">T    \n   </span><span style=\"background-color:none;color:red;\">C</span>-<span style=\"background-color:none;color:purple;\">g   \n </span><span style=\"background-color:none;color:fuchsia;\">G</span>---<span style=\"background-color:none;color:navy;\">--</span><span style=\"background-color:none;color:olive;\">c \n</span><span style=\"background-color:none;color:blue;\">A</span>----<span style=\"background-color:none;color:navy;\">---</span><span style=\"background-color:none;color:silver;\">t\n</span><span style=\"background-color:none;color:purple;\">g</span><span style=\"background-color:none;color:navy;\">---</span>----<span style=\"background-color:none;color:red;\">C\n </span><span style=\"background-color:none;color:olive;\">c</span><span style=\"background-color:none;color:navy;\">--</span>---<span style=\"background-color:none;color:fuchsia;\">G \n   </span><span style=\"background-color:none;color:yellow;\">t</span>-<span style=\"background-color:none;color:blue;\">A   \n    </span><span style=\"background-color:none;color:yellow;\">T    \n   </span><span style=\"background-color:none;color:red;\">C</span>-<span style=\"background-color:none;color:purple;\">g   </pre>\n<pre class=\"FLASK FLASK-f6 \" style=\"position:absolute;\">\n<span style=\"background-color:none;color:blue;\">A</span>----<span style=\"background-color:none;color:navy;\">---</span><span style=\"background-color:none;color:silver;\">t\n</span><span style=\"background-color:none;color:purple;\">g</span><span style=\"background-color:none;color:navy;\">---</span>----<span style=\"background-color:none;color:red;\">C\n </span><span style=\"background-color:none;color:olive;\">c</span><span style=\"background-color:none;color:navy;\">--</span>---<span style=\"background-color:none;color:fuchsia;\">G \n   </span><span style=\"background-color:none;color:silver;\">t</span>-<span style=\"background-color:none;color:blue;\">A   \n    </span><span style=\"background-color:none;color:yellow;\">T    \n   </span><span style=\"background-color:none;color:red;\">C</span>-<span style=\"background-color:none;color:purple;\">g   \n </span><span style=\"background-color:none;color:fuchsia;\">G</span>---<span style=\"background-color:none;color:navy;\">--</span><span style=\"background-color:none;color:olive;\">c \n</span><span style=\"background-color:none;color:blue;\">A</span>----<span style=\"background-color:none;color:navy;\">---</span><span style=\"background-color:none;color:silver;\">t\n</span><span style=\"background-color:none;color:purple;\">g</span><span style=\"background-color:none;color:navy;\">---</span>----<span style=\"background-color:none;color:red;\">C\n </span><span style=\"background-color:none;color:olive;\">c</span><span style=\"background-color:none;color:navy;\">--</span>---<span style=\"background-color:none;color:fuchsia;\">G \n   </span><span style=\"background-color:none;color:yellow;\">t</span>-<span style=\"background-color:none;color:blue;\">A   \n    </span><span style=\"background-color:none;color:yellow;\">T    \n   </span><span style=\"background-color:none;color:red;\">C</span>-<span style=\"background-color:none;color:purple;\">g   \n </span><span style=\"background-color:none;color:fuchsia;\">G</span>---<span style=\"background-color:none;color:navy;\">--</span><span style=\"background-color:none;color:olive;\">c </pre>\n<pre class=\"FLASK FLASK-f7 \" style=\"position:absolute;\">\n<span style=\"background-color:none;color:purple;\">g</span><span style=\"background-color:none;color:navy;\">---</span>----<span style=\"background-color:none;color:red;\">C\n </span><span style=\"background-color:none;color:olive;\">c</span><span style=\"background-color:none;color:navy;\">--</span>---<span style=\"background-color:none;color:fuchsia;\">G \n   </span><span style=\"background-color:none;color:silver;\">t</span>-<span style=\"background-color:none;color:blue;\">A   \n    </span><span style=\"background-color:none;color:yellow;\">T    \n   </span><span style=\"background-color:none;color:red;\">C</span>-<span style=\"background-color:none;color:purple;\">g   \n </span><span style=\"background-color:none;color:fuchsia;\">G</span>---<span style=\"background-color:none;color:navy;\">--</span><span style=\"background-color:none;color:olive;\">c \n</span><span style=\"background-color:none;color:blue;\">A</span>----<span style=\"background-color:none;color:navy;\">---</span><span style=\"background-color:none;color:silver;\">t\n</span><span style=\"background-color:none;color:purple;\">g</span><span style=\"background-color:none;color:navy;\">---</span>----<span style=\"background-color:none;color:red;\">C\n </span><span style=\"background-color:none;color:olive;\">c</span><span style=\"background-color:none;color:navy;\">--</span>---<span style=\"background-color:none;color:fuchsia;\">G \n   </span><span style=\"background-color:none;color:yellow;\">t</span>-<span style=\"background-color:none;color:blue;\">A   \n    </span><span style=\"background-color:none;color:yellow;\">T    \n   </span><span style=\"background-color:none;color:red;\">C</span>-<span style=\"background-color:none;color:purple;\">g   \n </span><span style=\"background-color:none;color:fuchsia;\">G</span>---<span style=\"background-color:none;color:navy;\">--</span><span style=\"background-color:none;color:olive;\">c \n</span><span style=\"background-color:none;color:blue;\">A</span>----<span style=\"background-color:none;color:navy;\">---</span><span style=\"background-color:none;color:silver;\">t</pre>\n</div>";
    #[test]
    fn test_flask(){
        let apple = rs3a::load(FLASK.to_string()).unwrap();
        assert_eq!(
            VALID_FLASK,
            render(&apple, None, None, "PREFIX", "FLASK"),
        );
    }
    #[test]
    fn test_flask_independent(){
        let apple = rs3a::load(FLASK.to_string()).unwrap();
        assert_eq!(
            VALID_FLASK_INDEPENDENT,
            render(&apple, None, Some(collection!{
                "independent".to_string() => "true".to_string(),
            }), "PREFIX", "FLASK"),
        );
    }
    const APPLE: &str = r###"
@ This work is licensed under the Creative Commons Attribution 4.0 International License. To view a copy of this license, visit
@ http://creativecommons.org/licenses/by/4.0/.
title just an apple
author DomesticMoth
width 12
height 5
loop true
delay 300
colors fg

  ,--./,-.  aa4444444444
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
|      (    4aaaaaaf4444
 \      `-, 4444444ffff4
  '._,._,'  444444444444

  ,--./,-.  444444444444
 / //,_.--' 444cc4444444
|   /  {    4444fffffff4
 \  \_,-`-, 4444fffffff4
  '._,._,'  444444444444

  ,--./,-.  444444444444
 '--._,.--' 444444444444
    }  {    fffffffffff4
 ,-'._,-`-, fffffffffff4
  '._,._,'  444444444444
    "###;
    const VALID_APPLE: &str = "<div class=\"PREFIXfg-red \" style=\"background-color:none;display:inline-block;position:relative;\" title=\"just an apple\" author=\"DomesticMoth\">\n<style type=\"text/css\">.APPLE{visibility:hidden;display:inline-block;top:0;left:0;margin:0;}.APPLE-f0{animation:APPLE-f0a 1.5s infinite;}@keyframes APPLE-f0a{0%{visibility:hidden;}10%{visibility:visible;}20%{visibility:hidden;}}.APPLE-f1{animation:APPLE-f1a 1.5s infinite;}@keyframes APPLE-f1a{20%{visibility:hidden;}30%{visibility:visible;}40%{visibility:hidden;}}.APPLE-f2{animation:APPLE-f2a 1.5s infinite;}@keyframes APPLE-f2a{40%{visibility:hidden;}50%{visibility:visible;}60%{visibility:hidden;}}.APPLE-f3{animation:APPLE-f3a 1.5s infinite;}@keyframes APPLE-f3a{60%{visibility:hidden;}70%{visibility:visible;}80%{visibility:hidden;}}.APPLE-f4{animation:APPLE-f4a 1.5s infinite;}@keyframes APPLE-f4a{80%{visibility:hidden;}90%{visibility:visible;}100%{visibility:hidden;}}</style>\n<pre class=\"APPLE APPLE-f0 \">\n  ,--./,-.  \n / <span class=\"PREFIXfg-bred \" style=\"background-color:none;\">//</span>     \\ \n|          |\n \\        / \n  '._,._,'  </pre>\n<pre class=\"APPLE APPLE-f1 \" style=\"position:absolute;\">\n  ,--./,-.  \n / <span class=\"PREFIXfg-bred \" style=\"background-color:none;\">//</span>    _\\ \n|      <span class=\"PREFIXfg-bwhite \" style=\"background-color:none;\"> /   \n</span> \\     <span class=\"PREFIXfg-bwhite \" style=\"background-color:none;\"> `-, \n</span>  '._,._,'  </pre>\n<pre class=\"APPLE APPLE-f2 \" style=\"position:absolute;\">\n  ,--./,-.  \n / <span class=\"PREFIXfg-bred \" style=\"background-color:none;\">//</span>   ,-' \n|      <span class=\"PREFIXfg-bwhite \" style=\"background-color:none;\">(    \n</span> \\     <span class=\"PREFIXfg-bwhite \" style=\"background-color:none;\"> `-, \n</span>  '._,._,'  </pre>\n<pre class=\"APPLE APPLE-f3 \" style=\"position:absolute;\">\n  ,--./,-.  \n / <span class=\"PREFIXfg-bred \" style=\"background-color:none;\">//</span>,_.--' \n|   <span class=\"PREFIXfg-bwhite \" style=\"background-color:none;\">/  {    \n</span> \\  <span class=\"PREFIXfg-bwhite \" style=\"background-color:none;\">\\_,-`-, \n</span>  '._,._,'  </pre>\n<pre class=\"APPLE APPLE-f4 \" style=\"position:absolute;\">\n  ,--./,-.  \n '--._,.--' \n<span class=\"PREFIXfg-bwhite \" style=\"background-color:none;\">    }  {    \n ,-'._,-`-, \n</span>  '._,._,'  </pre>\n</div>";
    const VALID_APPLE_INDEPENDENT: &str = "<div class=\"\" style=\"background-color:none;color:maroon;display:inline-block;independent:true;position:relative;\" title=\"just an apple\" author=\"DomesticMoth\">\n<style type=\"text/css\">.APPLE{visibility:hidden;display:inline-block;top:0;left:0;margin:0;}.APPLE-f0{animation:APPLE-f0a 1.5s infinite;}@keyframes APPLE-f0a{0%{visibility:hidden;}10%{visibility:visible;}20%{visibility:hidden;}}.APPLE-f1{animation:APPLE-f1a 1.5s infinite;}@keyframes APPLE-f1a{20%{visibility:hidden;}30%{visibility:visible;}40%{visibility:hidden;}}.APPLE-f2{animation:APPLE-f2a 1.5s infinite;}@keyframes APPLE-f2a{40%{visibility:hidden;}50%{visibility:visible;}60%{visibility:hidden;}}.APPLE-f3{animation:APPLE-f3a 1.5s infinite;}@keyframes APPLE-f3a{60%{visibility:hidden;}70%{visibility:visible;}80%{visibility:hidden;}}.APPLE-f4{animation:APPLE-f4a 1.5s infinite;}@keyframes APPLE-f4a{80%{visibility:hidden;}90%{visibility:visible;}100%{visibility:hidden;}}</style>\n<pre class=\"APPLE APPLE-f0 \">\n  ,--./,-.  \n / <span style=\"background-color:none;color:red;\">//</span>     \\ \n|          |\n \\        / \n  '._,._,'  </pre>\n<pre class=\"APPLE APPLE-f1 \" style=\"position:absolute;\">\n  ,--./,-.  \n / <span style=\"background-color:none;color:red;\">//</span>    _\\ \n|      <span style=\"background-color:none;color:white;\"> /   \n</span> \\     <span style=\"background-color:none;color:white;\"> `-, \n</span>  '._,._,'  </pre>\n<pre class=\"APPLE APPLE-f2 \" style=\"position:absolute;\">\n  ,--./,-.  \n / <span style=\"background-color:none;color:red;\">//</span>   ,-' \n|      <span style=\"background-color:none;color:white;\">(    \n</span> \\     <span style=\"background-color:none;color:white;\"> `-, \n</span>  '._,._,'  </pre>\n<pre class=\"APPLE APPLE-f3 \" style=\"position:absolute;\">\n  ,--./,-.  \n / <span style=\"background-color:none;color:red;\">//</span>,_.--' \n|   <span style=\"background-color:none;color:white;\">/  {    \n</span> \\  <span style=\"background-color:none;color:white;\">\\_,-`-, \n</span>  '._,._,'  </pre>\n<pre class=\"APPLE APPLE-f4 \" style=\"position:absolute;\">\n  ,--./,-.  \n '--._,.--' \n<span style=\"background-color:none;color:white;\">    }  {    \n ,-'._,-`-, \n</span>  '._,._,'  </pre>\n</div>";
    #[test]
    fn test_apple(){
        let apple = rs3a::load(APPLE.to_string()).unwrap();
        assert_eq!(
            VALID_APPLE,
            render(&apple, None, None, "PREFIX", "APPLE"),
        );
    }
    #[test]
    fn test_apple_independent(){
        let apple = rs3a::load(APPLE.to_string()).unwrap();
        assert_eq!(
            VALID_APPLE_INDEPENDENT,
            render(&apple, None, Some(collection!{
                "independent".to_string() => "true".to_string(),
            }), "PREFIX", "APPLE"),
        );
    }
}

/*pub fn new() -> Self {
    Self{ rng: StdRng::seed_from_u64(0) }
}
fn uid(&mut self) -> String {
    base64::encode(self.rng.gen::<u128>().to_be_bytes())
}*/
pub fn render(art: &Art, palette: Option<Palette>, styles: Option<HashMap<String, String>>, prefix: &str, uid: &str) -> String {
    let styles: HashMap<String, String> = match styles {
        Some(styles) => styles,
        None => { HashMap::new() }
    };
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
    let mut cl = class.len();
    colorise(ColorType::Fg(preart.base_fg), color_mod, &palette, independent, true, prefix, &mut css, &mut class);
    if let None = css.get(&"color".to_string()) {
        if class.len() == cl {
            css.insert("color".to_string(), "none".to_string());
        }
    }
    cl = class.len();
    colorise(ColorType::Bg(preart.base_bg), color_mod, &palette, independent, true, prefix, &mut css, &mut class);
    if let None = css.get(&"background-color".to_string()) {
        if class.len() == cl {
            css.insert("background-color".to_string(), "none".to_string());
        }
    }
    let mut anim_styles = String::new();
    if !only_preview {
        let anim_time = (art.body.frames.len() as f64 * art.header.delay as f64) / 1000.0;
        for i in 0..art.body.frames.len() {
            let stp: f64 = (100.0 / art.body.frames.len() as f64) * i as f64;
            let mp: f64 = (100.0 / art.body.frames.len() as f64) * (i as f64 + 0.5);
            let ep: f64 = (100.0 / art.body.frames.len() as f64) * (i + 1) as f64;
            anim_styles = format!("{}\
                .{uid}-f{nom}{{animation:{uid}-f{nom}a {time}s{inf};}}\
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
