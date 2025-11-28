use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};

use crate::{
    clause::{Fields, From, GroupBy, Having, Where},
    keyword,
    parse_option::ParseOption,
    quote_option::QuoteOption,
    visitor::Visitor,
};

pub struct Select {
    pub select_keyword: keyword::select,
    pub fields: Fields,
}

impl Select {
    pub fn accept<'a>(&'a self, visitor: &mut impl Visitor<'a>) {
        self.fields.accept(visitor);
    }
}

impl Parse for Select {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            select_keyword: input.call(keyword::select::parse_autocomplete)?,
            fields: input.parse()?,
        })
    }
}

impl ParseOption for Select {
    fn peek(input: ParseStream) -> bool {
        input.peek(keyword::select)
    }
}

impl ToTokens for Select {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let fields = &self.fields;
        quote! {
            ::kosame::repr::clause::Select::new(#fields)
        }
        .to_tokens(tokens);
    }
}

pub struct SelectCore {
    pub select: Select,
    pub from: Option<From>,
    pub r#where: Option<Where>,
    pub group_by: Option<GroupBy>,
    pub having: Option<Having>,
}

impl SelectCore {
    pub fn accept<'a>(&'a self, visitor: &mut impl Visitor<'a>) {
        self.select.accept(visitor);
        if let Some(inner) = self.from.as_ref() {
            inner.accept(visitor);
        }
        if let Some(inner) = self.r#where.as_ref() {
            inner.accept(visitor);
        }
        if let Some(inner) = self.group_by.as_ref() {
            inner.accept(visitor);
        }
        if let Some(inner) = self.having.as_ref() {
            inner.accept(visitor);
        }
    }
}

impl Parse for SelectCore {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            select: input.parse()?,
            from: input.call(From::parse_option)?,
            r#where: input.call(Where::parse_option)?,
            group_by: input.call(GroupBy::parse_option)?,
            having: input.call(Having::parse_option)?,
        })
    }
}

impl ParseOption for SelectCore {
    fn peek(input: ParseStream) -> bool {
        Select::peek(input)
    }
}

impl ToTokens for SelectCore {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let select = &self.select;
        let from = QuoteOption::from(&self.from);
        let r#where = QuoteOption::from(&self.r#where);
        let group_by = QuoteOption::from(&self.group_by);
        let having = QuoteOption::from(&self.having);

        quote! {
            ::kosame::repr::clause::SelectCore::new(
                #select,
                #from,
                #r#where,
                #group_by,
                #having,
            )
        }
        .to_tokens(tokens);
    }
}
