use proc_macro2::extra::DelimSpan;

use crate::pretty::{BreakMode, Printer};

pub trait Delim {
    fn pretty_print(
        &self,
        printer: &mut Printer<'_>,
        break_mode: BreakMode,
        f: impl FnOnce(&mut Printer<'_>),
    ) {
        printer.flush_trivia(self.span().open().into());
        printer.scan_text(self.open_text());
        printer.scan_begin(break_mode);
        f(printer);
        printer.flush_trivia(self.span().close().into());
        printer.scan_end();
        printer.scan_text(self.close_text());
    }

    #[must_use]
    fn open_text(&self) -> &'static str;

    #[must_use]
    fn close_text(&self) -> &'static str;

    #[must_use]
    fn span(&self) -> DelimSpan;
}

impl Delim for syn::token::Paren {
    fn open_text(&self) -> &'static str {
        "("
    }

    fn close_text(&self) -> &'static str {
        ")"
    }

    fn span(&self) -> DelimSpan {
        self.span
    }
}

impl Delim for syn::token::Bracket {
    fn open_text(&self) -> &'static str {
        "["
    }

    fn close_text(&self) -> &'static str {
        "]"
    }

    fn span(&self) -> DelimSpan {
        self.span
    }
}

impl Delim for syn::token::Brace {
    fn open_text(&self) -> &'static str {
        "{"
    }

    fn close_text(&self) -> &'static str {
        "}"
    }

    fn span(&self) -> DelimSpan {
        self.span
    }
}
