use pest::iterators::{Pair, Pairs};

use crate::loaders::load_expr;
use crate::marks::Syntax;
use crate::rule;

use super::IntoSyntax;
#[derive(Debug, serde::Serialize, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    fn load(pair: Pair<rule::Rule>) -> Self {
        let color = pair.into_inner().next().unwrap();
        match color.as_rule() {
            rule::Rule::hex_rgb_color => Self::load_hex_rgb(color.into_inner()),
            rule::Rule::rgb_color => Self::load_rgb_digit(color.into_inner()),
            rule::Rule::named_color => Self::load_raw_rgb(color),
            _ => unreachable!(),
        }
    }
}

impl IntoSyntax for Color {
    fn into_syn(pair: pest::iterators::Pair<crate::rule::Rule>) -> Vec<super::Syntax> {
        let mut inner = pair.into_inner();
        // 内部第一个参数为颜色
        let color = Self::load(inner.next().unwrap());
        //内部第二个参数为expr
        let expr = load_expr(inner.next().unwrap()).unwrap();

        vec![(color, expr).into()]
    }

    fn target_rule() -> crate::rule::Rule {
        rule::Rule::color_call
    }
}

impl From<(Color, Vec<Syntax>)> for Syntax {
    fn from((c, s): (Color, Vec<Syntax>)) -> Self {
        Self::Color { color: c, inner: s }
    }
}

impl Color {
    fn load_rgb_digit(mut inner: Pairs<rule::Rule>) -> Color {
        let r: u8 = inner.next().unwrap().as_str().trim().parse().unwrap();
        let g: u8 = inner.next().unwrap().as_str().trim().parse().unwrap();
        let b: u8 = inner.next().unwrap().as_str().trim().parse().unwrap();
        Color::new(r, g, b)
    }

    fn load_hex_rgb(mut inner: Pairs<rule::Rule>) -> Color {
        let r = inner.next().unwrap().as_str().trim();
        let r = u8::from_str_radix(r, 16).unwrap();
        let g = inner.next().unwrap().as_str().trim();
        let g = u8::from_str_radix(g, 16).unwrap();
        let b = inner.next().unwrap().as_str().trim();
        let b = u8::from_str_radix(b, 16).unwrap();
        Color { r, g, b }
    }

    fn load_raw_rgb(inner: Pair<rule::Rule>) -> Color {
        let inner_name = inner.as_str();
        Self::matcher(inner_name).expect(&format!("unknown Color Name :{}", inner_name))
    }
}

impl Color {
    //! color source from  [HTML Color Codes](https://html-color.codes/)
    crate::color_generate![
        RED(255, 0, 0),
        LIGHTSALMON(255, 160, 122),
        LIGHTCORAL(240, 128, 128),
        SALMON(250, 128, 114),
        DARKSALMON(233, 150, 122),
        TOMATO(255, 99, 71),
        INDIANRED(205, 92, 92),
        ORANGERED(255, 69, 0),
        CRIMSON(220, 20, 60),
        FIREBRICK(178, 34, 34),
        DARKRED(139, 0, 0),
        MAROON(128, 0, 0),
        BROWN(165, 42, 42),
        BISQUE(255, 228, 196),
        MOCCASIN(255, 228, 181),
        NAVAJOWHITE(255, 222, 173),
        SANDYBROWN(244, 164, 96),
        ROSYBROWN(188, 143, 143),
        PERU(205, 133, 63),
        CHOCOLATE(210, 105, 30),
        SIENNA(160, 82, 45),
        SADDLEBROWN(139, 69, 19),
        TAN(210, 180, 140),
        BLANCHEDALMOND(255, 235, 205),
        WHEAT(245, 222, 179),
        BURLYWOOD(222, 184, 135),
        ORANGE(255, 165, 0),
        PAPAYAWHIP(255, 239, 213),
        PEACHPUFF(255, 218, 185),
        CORAL(255, 127, 80),
        DARKORANGE(255, 140, 0),
        GOLD(255, 215, 0),
        LIGHTGOLDENRODYELLOW(250, 250, 210),
        PALEGOLDENROD(238, 232, 170),
        GOLDENROD(218, 165, 32),
        DARKGOLDENROD(184, 134, 11),
        YELLOW(255, 255, 0),
        LIGHTYELLOW(255, 255, 224),
        CORNSILK(255, 248, 220),
        LEMONCHIFFON(255, 250, 205),
        KHAKI(240, 230, 140),
        LIME(0, 255, 0),
        GREENYELLOW(173, 255, 47),
        CHARTREUSE(127, 255, 0),
        LIMEGREEN(50, 205, 50),
        LAWNGREEN(124, 252, 0),
        OLIVE(128, 128, 0),
        DARKKHAKI(189, 183, 107),
        YELLOWGREEN(154, 205, 50),
        OLIVEDRAB(107, 142, 35),
        DARKOLIVEGREEN(85, 107, 47),
        GREEN(0, 128, 0),
        PALEGREEN(152, 251, 152),
        LIGHTGREEN(144, 238, 144),
        DARKSEAGREEN(143, 188, 143),
        SPRINGGREEN(0, 255, 127),
        MEDIUMSPRINGGREEN(0, 250, 154),
        MEDIUMSEAGREEN(60, 179, 113),
        SEAGREEN(46, 139, 87),
        FORESTGREEN(34, 139, 34),
        DARKGREEN(0, 100, 0),
        TEAL(0, 128, 128),
        AQUAMARINE(127, 255, 212),
        MEDIUMAQUAMARINE(102, 205, 170),
        TURQUOISE(64, 224, 208),
        MEDIUMTURQUOISE(72, 209, 204),
        CADETBLUE(95, 158, 160),
        LIGHTSEAGREEN(32, 178, 170),
        CYAN(0, 255, 255),
        LIGHTCYAN(224, 255, 255),
        PALETURQUOISE(175, 238, 238),
        POWDERBLUE(176, 224, 230),
        DARKTURQUOISE(0, 206, 209),
        DARKCYAN(0, 139, 139),
        BLUE(0, 0, 255),
        LIGHTBLUE(173, 216, 230),
        LIGHTSTEELBLUE(176, 196, 222),
        LIGHTSKYBLUE(135, 206, 250),
        SKYBLUE(135, 206, 235),
        CORNFLOWERBLUE(100, 149, 237),
        ROYALBLUE(65, 105, 225),
        DODGERBLUE(30, 144, 255),
        DEEPSKYBLUE(0, 191, 255),
        STEELBLUE(70, 130, 180),
        MEDIUMBLUE(0, 0, 205),
        DARKBLUE(0, 0, 139),
        MIDNIGHTBLUE(25, 25, 112),
        NAVY(0, 0, 128),
        DARKSLATEBLUE(72, 61, 139),
        PURPLE(128, 0, 128),
        LAVENDER(230, 230, 250),
        THISTLE(216, 191, 216),
        PLUM(221, 160, 221),
        MEDIUMSLATEBLUE(123, 104, 238),
        MEDIUMPURPLE(147, 112, 219),
        SLATEBLUE(106, 90, 205),
        MEDIUMORCHID(186, 85, 211),
        BLUEVIOLET(138, 43, 226),
        DARKORCHID(153, 50, 204),
        DARKVIOLET(148, 0, 211),
        REBECCAPURPLE(102, 51, 153),
        INDIGO(75, 0, 130),
        MAGENTA(255, 0, 255),
        VIOLET(238, 130, 238),
        ORCHID(218, 112, 214),
        MEDIUMVIOLETRED(199, 21, 133),
        DARKMAGENTA(139, 0, 139),
        PINK(255, 192, 203),
        MISTYROSE(255, 228, 225),
        LIGHTPINK(255, 182, 193),
        HOTPINK(255, 105, 180),
        PALEVIOLETRED(219, 112, 147),
        DEEPPINK(255, 20, 147),
        GREY(128, 128, 128),
        GAINSBORO(220, 220, 220),
        LIGHTGREY(211, 211, 211),
        SILVER(192, 192, 192),
        DARKGREY(169, 169, 169),
        LIGHTSLATEGREY(119, 136, 153),
        SLATEGREY(112, 128, 144),
        DIMGREY(105, 105, 105),
        GHOSTWHITE(248, 248, 255),
        WHITESMOKE(245, 245, 245),
        WHITE(255, 255, 255),
        SNOW(255, 250, 250),
        MINTCREAM(245, 255, 250),
        ALICEBLUE(240, 248, 255),
        HONEYDEW(240, 255, 240),
        AZURE(240, 255, 255),
        LAVENDERBLUSH(255, 240, 245),
        SEASHELL(255, 245, 238),
        FLORALWHITE(255, 250, 240),
        IVORY(255, 255, 240),
        OLDLACE(253, 245, 230),
        LINEN(250, 240, 230),
        BEIGE(245, 245, 220),
        ANTIQUEWHITE(250, 235, 215),
        BLACK(0, 0, 0),
        DARKSLATEGREY(47, 79, 79)
    ];
}
#[macro_export]
macro_rules! color_generate {
    ($n:ident,$r:literal,$g:literal,$b:literal) => {
        const $n: Color = Color {
            r: $r,
            g: $g,
            b: $b,
        };
    };
    [$($n:ident($r:literal,$g:literal,$b:literal)),*]=>{
        $(
            crate::color_generate!($n,$r,$g,$b);
        )*

        fn matcher(input:&str)->Option<Color>{
            match input.to_uppercase().as_str() {
                $(stringify!($n)=>Some(Self::$n),)*
                _=>None
            }
        }
    }
}
