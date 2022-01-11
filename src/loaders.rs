use crate::{
    marks::{self, FontSize},
    pars_marker, Rule,
};
use pest::iterators::Pair;

pub(crate) fn load_expr(pair: Pair<Rule>) -> Option<Vec<marks::Syntax>> {
    if let Rule::expr = pair.as_rule() {
        Some(
            pair.into_inner()
                .into_iter()
                .map(|p| pars_marker(p))
                .collect(),
        )
    } else {
        None
    }
}

pub(crate) fn load_color(pair: Pair<Rule>) -> Option<(marks::Color, Vec<marks::Syntax>)> {
    if let Rule::color_call = pair.as_rule() {
        let mut inner = pair.into_inner();
        let color = inner.next().unwrap();
        let expr = inner.next().unwrap();

        // load color
        let color = if color.as_str().starts_with("(") {
            let mut inner = color.into_inner();
            let r: u8 = inner.next().unwrap().as_str().trim().parse().unwrap();
            let g: u8 = inner.next().unwrap().as_str().trim().parse().unwrap();
            let b: u8 = inner.next().unwrap().as_str().trim().parse().unwrap();
            marks::Color { r, g, b }
        } else {
            let mut inner = color.into_inner();

            let r = inner.next().unwrap().as_str().trim();
            let r = u8::from_str_radix(r, 16).unwrap();
            let g = inner.next().unwrap().as_str().trim();
            let g = u8::from_str_radix(g, 16).unwrap();
            let b = inner.next().unwrap().as_str().trim();
            let b = u8::from_str_radix(b, 16).unwrap();
            marks::Color { r, g, b }
        };

        // load syntax
        let syn = load_expr(expr)?;

        Some((color, syn))
    } else {
        None
    }
}

pub(crate) fn load_blob(pair: Pair<Rule>) -> Option<marks::Syntax> {
    if let Rule::blod_call = pair.as_rule() {
        Some(marks::Syntax::Blod(load_expr(
            pair.into_inner().next().unwrap(),
        )?))
    } else {
        None
    }
}

pub(crate) fn load_italic(pair: Pair<Rule>) -> Option<marks::Syntax> {
    if let Rule::italic_call = pair.as_rule() {
        Some(marks::Syntax::Italic(load_expr(
            pair.into_inner().next().unwrap(),
        )?))
    } else {
        None
    }
}

pub(crate) fn load_para(pair: Pair<Rule>) -> Option<marks::Syntax> {
    if let Rule::para_call = pair.as_rule() {
        Some(marks::Syntax::Paragraph)
    } else {
        None
    }
}

pub(crate) fn load_sep(pair: Pair<Rule>) -> Option<marks::Syntax> {
    if let Rule::sep_call = pair.as_rule() {
        Some(marks::Syntax::Seperate)
    } else {
        None
    }
}

pub(crate) fn load_string(pair: Pair<Rule>) -> Option<String> {
    if let Rule::string = pair.as_rule() {
        Some(pair.into_inner().next().unwrap().as_str().to_string())
    } else {
        None
    }
}

pub(crate) fn load_font_size(pair: Pair<Rule>) -> Option<(FontSize, Vec<marks::Syntax>)> {
    if let Rule::font_size_call = pair.as_rule() {
        let mut inner = pair.into_inner();
        let size = inner.next().unwrap().as_str().parse::<f64>().unwrap();
        let expr = inner.next().unwrap();
        let expr = load_expr(expr).unwrap();
        Some((FontSize { size }, expr))
    } else {
        None
    }
}

pub(crate) fn load_url(pair: Pair<Rule>) -> Option<marks::Syntax> {
    if let Rule::url_call = pair.as_rule() {
        let mut inner = pair.into_inner();

        let url = inner.next().unwrap().as_str().to_string();
        let name = inner.next().and_then(|s| load_expr(s));

        Some(marks::Syntax::Url { name, url })
    } else {
        None
    }
}

pub(crate) fn load_image(pair: Pair<Rule>) -> Option<marks::Syntax> {
    if let Rule::img_call = pair.as_rule() {
        let mut inner = pair.into_inner();

        let url = inner.next().unwrap().as_str().to_string();
        let name = inner
            .next()
            .and_then(|s| Some(s.into_inner().next().unwrap().as_str().to_string()));

        Some(marks::Syntax::Image { name, url })
    } else {
        None
    }
}
