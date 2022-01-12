use pest::iterators::Pair;

use crate::rule;

use self::{color::Color, font_size::FontSize};

pub mod color;
pub mod font_size;
pub mod plain;
pub mod simple_calls;

#[derive(Debug, serde::Serialize)]
#[serde(tag = "type", content = "inner")]
pub enum Syntax {
    Plain(String),
    Color {
        color: Color,
        inner: Vec<Syntax>,
    },
    FontSize {
        font_size: FontSize,
        inner: Vec<Syntax>,
    },
    Url {
        name: Option<Vec<Syntax>>,
        url: String,
    },
    Image {
        name: Option<Vec<Syntax>>,
        url: String,
    },
    Blod(Vec<Syntax>),
    Italic(Vec<Syntax>),
    Paragraph,
    Seperate,
}

pub(crate) trait IntoSyntax {
    fn into_syn(pair: Pair<rule::Rule>) -> Vec<Syntax>;
    fn target_rule() -> rule::Rule;
    fn checked_into(pair: Pair<rule::Rule>) -> Option<Vec<Syntax>> {
        match (Self::target_rule(), pair.as_rule()) {
            (x, y) if x == y => Some(Self::into_syn(pair)),
            _ => None,
        }
    }
}
#[macro_export]
macro_rules! non_data_mark {
    ($name:ident,$rule:path,$f:ident) => {
        pub struct $name;

        impl crate::marks::IntoSyntax for $name {
            fn into_syn(
                pair: pest::iterators::Pair<crate::rule::Rule>,
            ) -> Vec<crate::marks::Syntax> {
                vec![$f(pair)]
            }
            fn target_rule() -> crate::rule::Rule {
                $rule
            }
        }
    };

    ($name:ident,$rule:path,vec => $f:ident) => {
        pub struct $name;

        impl crate::marks::IntoSyntax for $name {
            fn into_syn(
                pair: pest::iterators::Pair<crate::rule::Rule>,
            ) -> Vec<crate::marks::Syntax> {
                $f(pair)
            }
            fn target_rule() -> crate::rule::Rule {
                $rule
            }
        }
    };
}
