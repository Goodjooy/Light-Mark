use crate::{
    marks::{
        self,
        color::Color,
        font_size::FontSize,
        simple_calls::{Blob, Group, Image, Italic, Pack, Paragraph, Seperate, Url},
        IntoSyntax,
    },
    Rule,
};
use pest::iterators::Pair;

pub(crate) fn load_expr(pair: Pair<Rule>) -> Option<Vec<marks::Syntax>> {
    if let Rule::expr = pair.as_rule() {
        pair.into_inner()
            .into_iter()
            .map(|p| parse_marker(p))
            .reduce(|mut l, r| {
                l.extend(r);
                l
            })
            .and_then(|s| Some(simplfy_expr(s)))
    } else {
        None
    }
}

pub(crate) fn parse_marker(pair: Pair<Rule>) -> Vec<marks::Syntax> {
    println!("{:?}",pair.as_rule());
    let handle = match pair.as_rule() {
        Rule::string => <String as IntoSyntax>::into_syn,
        Rule::color_call => <Color as IntoSyntax>::into_syn,
        Rule::blod_call => Blob::into_syn,
        Rule::italic_call => Italic::into_syn,
        Rule::para_call=>Paragraph::into_syn,
        Rule::sep_call => Seperate::into_syn,
        Rule::font_size_call => FontSize::into_syn,
        Rule::url_call => Url::into_syn,
        Rule::img_call => Image::into_syn,
        Rule::raw_call => Pack::into_syn,
        Rule::group_call => Group::into_syn,
        // secep
        Rule::any => Any::into_syn,
        Rule::EOI => Eoi::into_syn,
        //else
        _ => unreachable!(),
    };
    handle(pair)
}

crate::non_data_mark!(Any, Rule::any, on_reach_any);
fn on_reach_any(pair: Pair<Rule>) -> marks::Syntax {
    pair.as_str().to_string().into()
}

crate::non_data_mark!(Eoi,Rule::EOI,vec => on_eoi);
fn on_eoi(_pair: Pair<Rule>)->Vec<marks::Syntax>{
    vec![]
}


pub(crate) fn simplfy_expr(src: Vec<marks::Syntax>) -> Vec<marks::Syntax> {
    let mut res = Vec::with_capacity(src.len());
    let mut last = Option::<String>::None;
    for syn in src {
        if let marks::Syntax::Plain(s) = syn {
            if let Some(last) = &mut last {
                if s.len() == 0 {
                    last.push(' ');
                } else {
                    last.push_str(&s);
                }
            } else {
                last = Some(s)
            }
        } else {
            // check ok
            if let Some(ls) = last {
                if ls.len() != 0 {
                    res.push(ls.into());
                }
                last = None;
            }
            match syn {
                marks::Syntax::Color { color, inner } => {
                    res.push((color, simplfy_expr(inner)).into())
                }
                marks::Syntax::FontSize { font_size, inner } => {
                    res.push((font_size, simplfy_expr(inner)).into())
                }
                marks::Syntax::Blod(s) => res.push(marks::Syntax::Blod(simplfy_expr(s))),
                marks::Syntax::Italic(s) => res.push(marks::Syntax::Italic(simplfy_expr(s))),
                marks::Syntax::Url { name, url } => res.push(marks::Syntax::Url {
                    name: name.and_then(|f| Some(simplfy_expr(f))),
                    url,
                }),
                marks::Syntax::Plain(_) => unreachable!(),
                el => res.push(el),
            }
        }
    }
    // check ok
    if let Some(ls) = last {
        res.push(ls.into());
    }
    res
}
