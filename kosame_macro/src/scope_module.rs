use proc_macro_error::OptionExt;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Ident, Path};

use crate::{
    clause::{self, FromItem},
    part::TableAlias,
    path_ext::PathExt,
};

#[derive(Default, Clone)]
pub struct ScopeModule {
    tables: Vec<ScopeModuleItem>,
}

impl ScopeModule {
    pub fn new<'a>(from_items: impl IntoIterator<Item = &'a clause::FromItem>) -> Self {
        let mut tables = vec![];
        for from_item in from_items {
            fn collect(tables: &mut Vec<ScopeModuleItem>, item: &FromItem) {
                match item {
                    FromItem::Table { table, alias } => match alias {
                        Some(TableAlias {
                            name,
                            columns: Some(columns),
                            ..
                        }) => {
                            tables.push(ScopeModuleItem::Custom {
                                correlation: name.clone(),
                                columns: columns.columns.iter().cloned().collect(),
                            });
                        }
                        Some(TableAlias {
                            name,
                            columns: None,
                            ..
                        }) => {
                            tables.push(ScopeModuleItem::Aliased {
                                table: table.clone(),
                                alias: name.clone(),
                            });
                        }
                        None => {
                            tables.push(ScopeModuleItem::Existing(table.clone()));
                        }
                    },
                    FromItem::Subquery { command, alias, .. } => {
                        if let Some(alias) = alias {
                            if let Some(columns) = &alias.columns {
                                tables.push(ScopeModuleItem::Custom {
                                    correlation: alias.name.clone(),
                                    columns: columns.columns.iter().cloned().collect(),
                                });
                            } else {
                                tables.push(ScopeModuleItem::Custom {
                                    correlation: alias.name.clone(),
                                    columns: command
                                        .fields()
                                        .expect_or_abort("subquery must have return fields")
                                        .iter()
                                        .filter_map(|field| field.infer_name().cloned())
                                        .collect(),
                                });
                            }
                        }
                    }
                    FromItem::Join { left, right, .. } => {
                        collect(tables, left);
                        collect(tables, right);
                    }
                    FromItem::NaturalJoin { left, right, .. } => {
                        collect(tables, left);
                        collect(tables, right);
                    }
                    FromItem::CrossJoin { left, right, .. } => {
                        collect(tables, left);
                        collect(tables, right);
                    }
                }
            }
            collect(&mut tables, from_item);
        }
        Self { tables }
    }
}

impl ToTokens for ScopeModule {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let tables = &self.tables;
        let columns = self.tables.iter().map(|table| {
            let name = table.name();
            quote! {
                pub use super::tables::#name::columns::*;
            }
        });
        quote! {
            mod scope {
                pub mod tables {
                    #(#tables)*
                }
                pub mod columns {
                    #(#columns)*
                }
            }
        }
        .to_tokens(tokens);
    }
}

#[derive(Clone)]
enum ScopeModuleItem {
    Existing(Path),
    Aliased {
        table: Path,
        alias: Ident,
    },
    Custom {
        correlation: Ident,
        columns: Vec<Ident>,
    },
}

impl ScopeModuleItem {
    fn name(&self) -> &Ident {
        match self {
            Self::Existing(table) => &table.segments.last().expect("paths cannot be empty").ident,
            Self::Aliased { alias, .. } => alias,
            Self::Custom { correlation, .. } => correlation,
        }
    }
}

impl ToTokens for ScopeModuleItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ScopeModuleItem::Existing(table) => {
                let table = table.to_call_site(3);
                quote! { pub use #table; }
            }
            ScopeModuleItem::Aliased { table, alias } => {
                let table = table.to_call_site(4);
                let table_name = alias.to_string();
                quote! {
                    pub mod #alias {
                        pub const TABLE_NAME: &str = #table_name;
                        pub use #table::columns;
                    }
                }
            }
            ScopeModuleItem::Custom {
                correlation,
                columns,
            } => {
                let table_name = correlation.to_string();
                let column_strings = columns.iter().map(|column| column.to_string());
                quote! {
                    pub mod #correlation {
                        pub const TABLE_NAME: &str = #table_name;
                        pub mod columns {
                            #(
                                pub mod #columns {
                                    pub const COLUMN_NAME: &str = #column_strings;
                                }
                            )*
                        }
                    }
                }
            }
        }
        .to_tokens(tokens);
    }
}
