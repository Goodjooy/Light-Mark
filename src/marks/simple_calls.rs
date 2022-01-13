use pest::iterators::Pair;

use crate::{loaders::load_expr, non_data_mark, rule};

use super::Syntax;

non_data_mark!(Blob, rule::Rule::blod_call, on_blob_call);
fn on_blob_call(pair: Pair<rule::Rule>) -> super::Syntax {
    Syntax::Blod(load_expr(pair.into_inner().next().unwrap()).unwrap())
}

non_data_mark!(Italic, rule::Rule::italic_call, on_italic_call);
fn on_italic_call(pair: Pair<rule::Rule>) -> super::Syntax {
    Syntax::Italic(load_expr(pair.into_inner().next().unwrap()).unwrap())
}

non_data_mark!(Paragraph, rule::Rule::para_call, on_para_call);
fn on_para_call(_pair: Pair<rule::Rule>) -> Syntax {
    Syntax::Paragraph
}

non_data_mark!(Seperate, rule::Rule::sep_call, on_sep_call);
fn on_sep_call(_pair: Pair<rule::Rule>) -> Syntax {
    Syntax::Seperate
}

non_data_mark!(Url, rule::Rule::url_call, on_url_call);
fn on_url_call(pair: Pair<rule::Rule>) -> Syntax {
    let mut inner = pair.into_inner();
    // 第一个参数，网址，必选
    let url = inner.next().unwrap().as_str().to_string();
    // 名称，可选
    let name = inner.next().and_then(|f| load_expr(f));

    Syntax::Url { name, url }
}

non_data_mark!(Image, rule::Rule::img_call, on_image_call);
fn on_image_call(pair: Pair<rule::Rule>) -> Syntax {
    let mut inner = pair.into_inner();
    // 第一个参数，网址，必选
    let url = inner.next().unwrap().as_str().to_string();
    // 名称，可选
    let name = inner.next().and_then(load_expr);

    Syntax::Image { name, url }
}

non_data_mark!(Pack, rule::Rule::raw_call,vec => on_pack_call);
fn on_pack_call(pair: Pair<rule::Rule>) -> Vec<Syntax> {
    let mut inner = pair.into_inner();
    let mut res = Vec::with_capacity(3);

    // 左括号
    res.push(inner.next().unwrap().as_str().to_string().into());
    // 中间表达式
    let expr = inner.next().unwrap();
    if expr.as_rule() == rule::Rule::expr {
        res.extend(load_expr(expr).unwrap());
        // 右括号
        res.push(inner.next().unwrap().as_str().to_string().into());
    } else {
        res.push(expr.as_str().to_string().into())
    }

    res
}

non_data_mark!(Group, rule::Rule::group_call,vec => on_group_call);
fn on_group_call(pair: Pair<rule::Rule>) -> Vec<Syntax> {
    load_expr(pair.into_inner().next().unwrap()).unwrap()
}
