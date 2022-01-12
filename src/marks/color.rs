use pest::iterators::{Pair, Pairs};

use crate::loaders::load_expr;
use crate::marks::Syntax;
use crate::rule;

use super::IntoSyntax;
#[derive(Debug, serde::Serialize)]
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
}
