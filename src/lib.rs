use crate::rule::Rule;

use loaders::{parse_marker, simplfy_expr};
use pest::Parser;

extern crate pest;
#[macro_use]
extern crate pest_derive;

mod loaders;
mod marks;
mod rule;

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

pub(crate) fn inner_parser(input: &str) -> Result<Vec<marks::Syntax>, pest::error::Error<Rule>> {
    let res = rule::LightMarkParser::parse(Rule::root, input)?;

    let res = res
        .into_iter()
        .map(parse_marker)
        .reduce(|mut l, r| {
            l.extend(r);
            l
        })
        .and_then(|f| Some(simplfy_expr(f)))
        .unwrap_or_default();

    Ok(res)
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
        let s = "color(#ffaa12,对，的)，颜色（red，好）";

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
    fn test_sep_call() {
        let s = "分段（）分割线（）aa";

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
(())
也没有什么特别的emm 好吧 加粗（斜体（颜色（#FFFFFF, 非常重要！！）））

(好欸 （（补充说明））)
url(https://pest.rs/book/grammars/syntax.html, 颜色（#ABCDEF,好欸）)
图片（https://pest.rs/book/grammars/syntax.html，"怎么样？"）"#;
        let res = inner_parser(s);
        let res = res.unwrap();
        let jv = serde_json::to_string_pretty(&res).unwrap();

        println!("{}", jv)
    }
}
