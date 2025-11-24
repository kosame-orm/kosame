use proc_macro2::extra::DelimSpan;

pub enum Delimiter<'a> {
    Paren(&'a syn::token::Paren),
    Brace(&'a syn::token::Brace),
    Bracket(&'a syn::token::Bracket),
    None,
}

impl Delimiter<'_> {
    fn span(&self) -> Option<DelimSpan> {
        match self {
            Self::Paren(inner) => Some(inner.span),
            Self::Brace(inner) => Some(inner.span),
            Self::Bracket(inner) => Some(inner.span),
            Self::None => None,
        }
    }
}

//
// use proc_macro2::extra::DelimSpan;
//
// use crate::pretty::Text;
//
// pub trait Delimiter {
//     fn span(&self) -> DelimSpan;
// }
//
// impl Delimiter for syn::token::Paren {
//     fn span(&self) -> DelimSpan {
//         self.span
//     }
// }
//
// impl Delimiter for syn::token::Brace {
//     fn span(&self) -> DelimSpan {
//         self.span
//     }
// }
//
// impl Delimiter for syn::token::Bracket {
//     fn span(&self) -> DelimSpan {
//         self.span
//     }
// }
//
// pub enum Delim<'a> {
//     Open(&'a dyn Delimiter),
//     Close(&'a dyn Delimiter),
// }
//
// impl Text for Delim<'_> {
//     fn span(&self) -> Option<proc_macro2::Span> {
//         match self {
//             Self::Open(inner) => inner.
//         }
//     }
// }
