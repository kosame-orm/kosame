use std::fmt::Write;

use crate::{command::Command, expr::Expr, part::TableAlias};

pub struct From<'a> {
    item: FromItem<'a>,
}

impl<'a> From<'a> {
    #[inline]
    pub const fn new(item: FromItem<'a>) -> Self {
        Self { item }
    }

    #[inline]
    pub const fn item(&self) -> &FromItem<'a> {
        &self.item
    }
}

impl kosame_sql::FmtSql for From<'_> {
    fn fmt_sql<D>(&self, formatter: &mut kosame_sql::Formatter<D>) -> kosame_sql::Result
    where
        D: kosame_sql::Dialect,
    {
        formatter.write_str(" from ")?;
        self.item.fmt_sql(formatter)?;

        Ok(())
    }
}

pub enum JoinType {
    Inner,
    Left,
    Right,
    Full,
}

impl kosame_sql::FmtSql for JoinType {
    fn fmt_sql<D>(&self, formatter: &mut kosame_sql::Formatter<D>) -> kosame_sql::Result
    where
        D: kosame_sql::Dialect,
    {
        match self {
            Self::Inner => formatter.write_str(" inner join "),
            Self::Left => formatter.write_str(" left join "),
            Self::Right => formatter.write_str(" right join "),
            Self::Full => formatter.write_str(" full join "),
        }
    }
}

pub enum FromItem<'a> {
    Table {
        table: &'a str,
        alias: Option<TableAlias<'a>>,
    },
    Subquery {
        lateral: bool,
        command: &'a Command<'a>,
        alias: Option<TableAlias<'a>>,
    },
    Join {
        left: &'a FromItem<'a>,
        join_type: JoinType,
        right: &'a FromItem<'a>,
        on: Expr<'a>,
    },
    NaturalJoin {
        left: &'a FromItem<'a>,
        join_type: JoinType,
        right: &'a FromItem<'a>,
    },
    CrossJoin {
        left: &'a FromItem<'a>,
        right: &'a FromItem<'a>,
    },
}

impl kosame_sql::FmtSql for FromItem<'_> {
    fn fmt_sql<D>(&self, formatter: &mut kosame_sql::Formatter<D>) -> kosame_sql::Result
    where
        D: kosame_sql::Dialect,
    {
        match self {
            Self::Table { table, alias } => {
                formatter.write_ident(table)?;
                if let Some(alias) = alias {
                    formatter.write_str(" as ")?;
                    alias.fmt_sql(formatter)?;
                }
            }
            Self::Subquery {
                lateral,
                command: select,
                alias,
            } => {
                if *lateral {
                    formatter.write_str("lateral ")?;
                }
                formatter.write_str("(")?;
                select.fmt_sql(formatter)?;
                formatter.write_str(")")?;
                if let Some(alias) = alias {
                    formatter.write_str(" as ")?;
                    alias.fmt_sql(formatter)?;
                }
            }
            Self::Join {
                left,
                join_type,
                right,
                on,
            } => {
                left.fmt_sql(formatter)?;
                join_type.fmt_sql(formatter)?;
                right.fmt_sql(formatter)?;
                formatter.write_str(" on ")?;
                on.fmt_sql(formatter)?;
            }
            Self::NaturalJoin {
                left,
                join_type,
                right,
            } => {
                left.fmt_sql(formatter)?;
                formatter.write_str(" natural")?;
                join_type.fmt_sql(formatter)?;
                right.fmt_sql(formatter)?;
            }
            Self::CrossJoin { left, right } => {
                left.fmt_sql(formatter)?;
                formatter.write_str(" cross join ")?;
                right.fmt_sql(formatter)?;
            }
        }

        Ok(())
    }
}
