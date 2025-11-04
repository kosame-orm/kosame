use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Ident, Path};

use crate::{
    clause::{FromItem, WithItem},
    command::Command,
    part::TableAlias,
    path_ext::PathExt,
};

pub struct Scopes<'a> {
    scopes: Vec<Scope<'a>>,
    with_modules: Vec<WithModule<'a>>,
}

impl ToTokens for Scopes<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let scopes = &self.scopes;
        let with_modules = &self.with_modules;
        quote! {
            mod scopes {
                #(#with_modules)*
                #(#scopes)*
            }
        }
        .to_tokens(tokens);
    }
}

struct Scope<'a> {
    index: usize,
    modules: Vec<ScopeModule<'a>>,
}

impl<'a> Scope<'a> {
    fn new(index: usize, modules: Vec<ScopeModule<'a>>) -> Self {
        Self { index, modules }
    }
}

impl ToTokens for Scope<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = format_ident!("scope_{}", self.index);
        let tables = &self.modules;
        let columns = self
            .modules
            .iter()
            .filter(|module| !module.is_inherited())
            .map(|module| module.name());
        quote! {
            pub mod #name {
                pub mod tables {
                    #(#tables)*
                }
                pub mod columns {
                    #(pub use super::tables::#columns::columns::*;)*
                }
            }
        }
        .to_tokens(tokens);
    }
}

enum ScopeModule<'a> {
    Table {
        path: &'a Path,
        alias: Option<&'a Ident>,
    },
    Custom {
        name: &'a Ident,
        columns: Vec<CustomColumn<'a>>,
    },
    Inherited {
        index: usize,
        name: &'a Ident,
    },
}

impl<'a> ScopeModule<'a> {
    fn name(&self) -> &'a Ident {
        match self {
            Self::Table {
                alias: Some(alias), ..
            } => alias,
            Self::Table { path, .. } => &path.segments.last().expect("path cannot be empty").ident,
            Self::Custom { name, .. } => name,
            Self::Inherited { name, .. } => name,
        }
    }

    fn is_inherited(&self) -> bool {
        matches!(self, Self::Inherited { .. })
    }
}

impl ToTokens for ScopeModule<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Table {
                path,
                alias: Some(alias),
            } => {
                let path = path.to_call_site(4);
                let table_name = alias.to_string();
                quote! {
                    pub mod #alias {
                        pub const TABLE_NAME: &str = #table_name;
                        pub use #path::columns;
                    }
                }
            }
            Self::Table { path, .. } => {
                let path = path.to_call_site(3);
                quote! {
                    pub use #path;
                }
            }
            Self::Custom { name, columns } => {
                let name_string = name.to_string();
                quote! {
                    pub mod #name {
                        pub const TABLE_NAME: &str = #name_string;
                        pub mod columns {
                            #(#columns)*
                        }
                    }
                }
            }
            Self::Inherited { index, name } => {
                let scope_name = format_ident!("scope_{}", index);
                quote! {
                    pub use super::super::#scope_name::#name;
                }
            }
        }
        .to_tokens(tokens);
    }
}

struct CustomColumn<'a> {
    name: &'a Ident,
}

impl ToTokens for CustomColumn<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let name_string = self.name.to_string();
        quote! {
            pub mod #name {
                pub const COLUMN_NAME: &str = #name_string;
            }
        }
        .to_tokens(tokens);
    }
}

struct WithModule<'a> {
    index: usize,
    item: &'a WithItem,
}

impl ToTokens for WithModule<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let module_name = format_ident!("with_{}", self.index);
        let table_name_string = self.item.alias.name.to_string();
        let columns = self
            .item
            .command
            .command_type
            .fields()
            .into_iter()
            .flat_map(|fields| {
                fields
                    .iter()
                    .flat_map(|field| field.infer_name().map(|name| CustomColumn { name }))
            });
        quote! {
            mod #module_name {
                pub const TABLE_NAME: &str = #table_name_string;
                pub mod columns {
                    #(#columns)*
                }
            }
        }
        .to_tokens(tokens);
    }
}

impl<'a> From<&'a Command> for Scopes<'a> {
    fn from(value: &'a Command) -> Self {
        fn inner<'a>(
            global_scope_index: &mut usize,
            global_with_index: &mut usize,
            scopes: &mut Vec<Scope<'a>>,
            with_modules: &mut Vec<WithModule<'a>>,
            command: &'a Command,
            inherited_with_items: &mut Vec<(usize, &'a Ident)>,
            inherited_from_items: &mut Vec<(usize, &'a Ident)>,
        ) {
            let scope_index = *global_scope_index;
            *global_scope_index += 1;
            let mut shadow = HashSet::new();

            let mut inherited_with_items_count = 0;
            if let Some(with) = &command.with {
                for item in with.items.iter() {
                    inner(
                        global_scope_index,
                        global_with_index,
                        scopes,
                        with_modules,
                        &item.command,
                        inherited_with_items,
                        inherited_from_items,
                    );

                    inherited_with_items.push((*global_with_index, &item.alias.name));
                    inherited_with_items_count += 1;

                    with_modules.push(WithModule {
                        index: *global_with_index,
                        item,
                    });
                    *global_with_index += 1;
                }
            }

            let mut modules = Vec::new();
            if let Some(target_table) = command.target_table() {
                let module = ScopeModule::Table {
                    path: &target_table.table,
                    alias: target_table.alias.as_ref().map(|alias| &alias.ident),
                };
                shadow.insert(module.name());
                modules.push(module);
            }

            let mut inherited_from_items_count = 0;
            if let Some(from_item) = command.from_item() {
                for from_item in from_item {
                    let module = match from_item {
                        FromItem::Table { table, alias, .. } => {
                            let mut table = table;

                            match alias {
                                Some(TableAlias {
                                    name,
                                    columns: Some(columns),
                                    ..
                                }) => Some(ScopeModule::Custom {
                                    name,
                                    columns: columns
                                        .columns
                                        .iter()
                                        .map(|name| CustomColumn { name })
                                        .collect(),
                                }),
                                _ => Some(ScopeModule::Table {
                                    path: table,
                                    alias: alias.as_ref().map(|alias| &alias.name),
                                }),
                            }
                        }
                        FromItem::Subquery {
                            lateral_keyword,
                            command,
                            alias,
                            ..
                        } => {
                            let mut clean_from_items = Vec::new();
                            inner(
                                global_scope_index,
                                global_with_index,
                                scopes,
                                with_modules,
                                command,
                                inherited_with_items,
                                match lateral_keyword {
                                    Some(..) => inherited_from_items,
                                    None => &mut clean_from_items,
                                },
                            );
                            alias.as_ref().map(|alias| ScopeModule::Custom {
                                name: &alias.name,
                                columns: alias
                                    .columns
                                    .as_ref()
                                    .map(|columns| {
                                        columns
                                            .columns
                                            .iter()
                                            .map(|name| CustomColumn { name })
                                            .collect()
                                    })
                                    .unwrap_or(Vec::new()),
                            })
                        }
                        _ => None,
                    };
                    if let Some(module) = module {
                        shadow.insert(module.name());
                        inherited_from_items.push((scope_index, module.name()));
                        inherited_from_items_count += 1;

                        modules.push(module);
                    }
                }
            }

            for (source_index, name) in inherited_from_items.iter() {
                if !shadow.contains(name) {
                    modules.push(ScopeModule::Inherited {
                        index: *source_index,
                        name,
                    });
                }
            }

            inherited_with_items.truncate(inherited_with_items.len() - inherited_with_items_count);
            inherited_from_items.truncate(inherited_from_items.len() - inherited_from_items_count);

            scopes.push(Scope::new(scope_index, modules));
        }

        let mut scopes = Vec::new();
        let mut with_modules = Vec::new();
        inner(
            &mut 0,
            &mut 0,
            &mut scopes,
            &mut with_modules,
            value,
            &mut Vec::new(),
            &mut Vec::new(),
        );
        scopes.reverse();
        Scopes {
            scopes,
            with_modules,
        }
    }
}
