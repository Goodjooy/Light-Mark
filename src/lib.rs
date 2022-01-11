use loaders::{
    load_blob, load_color, load_font_size, load_image, load_italic, load_para, load_sep,
    load_string, load_url, load_raw,
};
use pest::{iterators::Pair, Parser};

use crate::loaders::load_expr;

extern crate pest;
#[macro_use]
extern crate pest_derive;

mod loaders;
mod marks;

pub enum PaeseError {
    Pest(pest::error::Error<Rule>),
    Json(serde_json::error::Error),
}

impl From<pest::error::Error<Rule>> for PaeseError {
    fn from(e: pest::error::Error<Rule>) -> Self {
        Self::Pest(e)
    }
}
impl From<serde_json::error::Error> for PaeseError {
    fn from(e: serde_json::error::Error) -> Self {
        Self::Json(e)
    }
}

pub fn parse(input: &str) -> Result<serde_json::Value, PaeseError> {
    let res = inner_parser(input)?;
    let res = serde_json::to_value(&res)?;
    Ok(res)
}

#[derive(Parser)]
#[grammar = "../syn/syntax.pest"]
struct LightMarkParser;

pub(crate) fn inner_parser(input: &str) -> Result<Vec<marks::Syntax>, pest::error::Error<Rule>> {
    let mut res = LightMarkParser::parse(Rule::root, input)?;

    let res = load_expr(res.next().unwrap()).unwrap();
    Ok(simplfy_expr(res))
}

fn pars_marker(pair: Pair<Rule>) -> marks::Syntax {
    match pair.as_rule() {
        Rule::string => load_string(pair).unwrap().into(),
        Rule::color_call => load_color(pair).unwrap().into(),
        Rule::blod_call => load_blob(pair).unwrap(),
        Rule::italic_call => load_italic(pair).unwrap(),
        Rule::para_call => load_para(pair).unwrap(),
        Rule::sep_call => load_sep(pair).unwrap(),
        Rule::font_size_call => load_font_size(pair).unwrap().into(),
        Rule::url_call => load_url(pair).unwrap(),
        Rule::img_call => load_image(pair).unwrap(),
        Rule::raw_call=>load_raw(pair).unwrap(),
        // secep
        Rule::any => pair.as_str().trim().to_string().into(),
        Rule::func_call => pars_marker(pair.into_inner().next().unwrap()),
        //else
        _ => unreachable!(),
    }
}

fn simplfy_expr(src: Vec<marks::Syntax>) -> Vec<marks::Syntax> {
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
                marks::Syntax::Muli(inner)=>{
                    res.extend(simplfy_expr(inner))
                }
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

#[cfg(test)]
mod test {

    use crate::inner_parser;

    #[test]
    fn test_raw() {
        let s = "aa（（ababbbv`abba`“我不是在”））";

        let res = inner_parser(s);
        assert!(res.is_ok());
        let res = res.unwrap();
        let jv = serde_json::to_string_pretty(&res).unwrap();

        println!("{}", jv)
    }

    #[test]
    fn test_plain() {
        let s = "ababbbv`abba`“我不是在”";

        let res = inner_parser(s);
        assert!(res.is_ok());
        let res = res.unwrap();
        let jv = serde_json::to_string_pretty(&res).unwrap();

        println!("{}", jv)
    }

    #[test]
    fn test_color_call() {
        let s = "color(#ffaa12,对，的)，颜色（（1，1，1），好）";

        let res = inner_parser(s);
        assert!(res.is_ok());
        let res = res.unwrap();
        let jv = serde_json::to_string_pretty(&res).unwrap();

        println!("{}", jv)
    }

    #[test]
    fn test_blob_call() {
        let s = "blod(加粗)加粗（要加粗)";

        let res = inner_parser(s);
        let res = res.unwrap();
        let jv = serde_json::to_string_pretty(&res).unwrap();

        println!("{}", jv)
    }

    #[test]
    fn test_full() {
        let s = r#"这是一条通知
        颜色（（11，22，32），特别注意）
        分段（）
        分割线（）
        也没有什么特别的emm 好吧 加粗（斜体（颜色（#FFFFFF, 非常重要！！）））
        分段（）
        
        url(https://pest.rs/book/grammars/syntax.html, 颜色（#ABCDEF,好欸）)
        图片（https://pest.rs/book/grammars/syntax.html，"怎么样？"）"#;
        let res = inner_parser(s);
        let res = res.unwrap();
        let jv = serde_json::to_string_pretty(&res).unwrap();

        println!("{}", jv)
    }
}
