use crate::rule;

use super::{IntoSyntax, Syntax};

impl IntoSyntax for String {
    fn into_syn(pair: pest::iterators::Pair<crate::rule::Rule>) -> Vec<Syntax> {
        vec![pair
            .into_inner()
            .next()
            .unwrap()
            .as_str()
            .to_string()
            .into()]
    }
    fn target_rule() -> rule::Rule {
        rule::Rule::string
    }
}

impl From<String> for Syntax {
    fn from(s: String) -> Self {
        Self::Plain(s)
    }
}
