use syn::{
    braced, bracketed, parenthesized,
    parse::{Parse, ParseStream},
};

use crate::pretty::{BreakMode, PrettyPrint, Printer, Text};

pub enum Macro<T> {
    Parenthesized {
        paren: syn::token::Paren,
        inner: T,
    },
    Braced {
        brace: syn::token::Brace,
        inner: T,
    },
    Bracketed {
        bracket: syn::token::Bracket,
        inner: T,
    },
}

impl<T> Parse for Macro<T>
where
    T: Parse,
{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        let content;
        if lookahead.peek(syn::token::Paren) {
            Ok(Self::Parenthesized {
                paren: parenthesized!(content in input),
                inner: content.parse()?,
            })
        } else if lookahead.peek(syn::token::Brace) {
            Ok(Self::Braced {
                brace: braced!(content in input),
                inner: content.parse()?,
            })
        } else if lookahead.peek(syn::token::Bracket) {
            Ok(Self::Bracketed {
                bracket: bracketed!(content in input),
                inner: content.parse()?,
            })
        } else {
            Err(lookahead.error())
        }
    }
}

impl<T> PrettyPrint for Macro<T>
where
    T: PrettyPrint,
{
    fn pretty_print(&self, printer: &mut Printer<'_>) {
        match self {
            Self::Parenthesized { paren, inner } => {
                printer.scan_begin(Some(paren.into()), BreakMode::Consistent);
                inner.pretty_print(printer);
                printer.scan_end(Some(paren.into()));
            }
            Self::Braced { brace, inner } => {
                printer.scan_begin(Some(brace.into()), BreakMode::Consistent);
                printer.scan_text(Text::new(" ", None, super::TextMode::NoBreak));
                inner.pretty_print(printer);
                printer.scan_text(Text::new(" ", None, super::TextMode::NoBreak));
                printer.scan_end(Some(brace.into()));
            }
            Self::Bracketed { bracket, inner } => {
                printer.scan_begin(Some(bracket.into()), BreakMode::Consistent);
                inner.pretty_print(printer);
                printer.scan_end(Some(bracket.into()));
            }
        }
    }
}
