use std::vec;

use crate::{loaders::load_expr, rule};

use super::{IntoSyntax, Syntax};

#[derive(Debug, serde::Serialize)]
pub struct FontSize {
    pub size: f64,
}

impl IntoSyntax for FontSize {
    fn into_syn(pair: pest::iterators::Pair<crate::rule::Rule>) -> Vec<super::Syntax> {
        let mut inner = pair.into_inner();
        //第一个参数是字体大小
        let size: f64 = inner.next().unwrap().as_str().trim().parse().unwrap();
        // 第二个是要被调整的表达式
        let expr = load_expr(inner.next().unwrap()).unwrap();

        vec![(FontSize { size }, expr).into()]
    }

    fn target_rule() -> crate::rule::Rule {
        rule::Rule::font_size_call
    }
}

impl From<(FontSize, Vec<Syntax>)> for Syntax {
    fn from((f, s): (FontSize, Vec<Syntax>)) -> Self {
        Self::FontSize {
            font_size: f,
            inner: s,
        }
    }
}
