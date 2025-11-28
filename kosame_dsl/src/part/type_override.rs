use syn::{
    Path, Token,
    parse::{Parse, ParseStream},
};

use crate::parse_option::ParseOption;

pub struct TypeOverride {
    pub colon_token: Token![:],
    pub type_path: Path,
}

impl ParseOption for TypeOverride {
    fn peek(input: ParseStream) -> bool {
        input.peek(Token![:])
    }
}

impl Parse for TypeOverride {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            colon_token: input.parse()?,
            type_path: input.parse()?,
        })
    }
}
