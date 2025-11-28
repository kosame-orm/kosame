use std::fmt::Write;

use crate::clause::{self, Fields, From, GroupBy, Having, Where};

pub struct Select<'a> {
    fields: Fields<'a>,
}

impl<'a> Select<'a> {
    #[inline]
    #[must_use]
    pub const fn new(fields: Fields<'a>) -> Self {
        Self { fields }
    }

    #[inline]
    #[must_use]
    pub const fn fields(&self) -> &Fields<'a> {
        &self.fields
    }
}

impl kosame_sql::FmtSql for Select<'_> {
    fn fmt_sql<D>(&self, formatter: &mut kosame_sql::Formatter<D>) -> kosame_sql::Result
    where
        D: kosame_sql::Dialect,
    {
        formatter.write_str("select ")?;
        self.fields.fmt_sql(formatter)?;
        Ok(())
    }
}

pub struct SelectCore<'a> {
    #[allow(clippy::struct_field_names)]
    select: clause::Select<'a>,
    from: Option<From<'a>>,
    r#where: Option<Where<'a>>,
    group_by: Option<GroupBy<'a>>,
    having: Option<Having<'a>>,
}

impl<'a> SelectCore<'a> {
    #[inline]
    #[must_use]
    pub const fn new(
        select: clause::Select<'a>,
        from: Option<From<'a>>,
        r#where: Option<Where<'a>>,
        group_by: Option<GroupBy<'a>>,
        having: Option<Having<'a>>,
    ) -> Self {
        Self {
            select,
            from,
            r#where,
            group_by,
            having,
        }
    }

    #[inline]
    #[must_use]
    pub const fn select(&self) -> &clause::Select<'a> {
        &self.select
    }

    #[inline]
    #[must_use]
    pub const fn from(&self) -> Option<&From<'a>> {
        self.from.as_ref()
    }

    #[inline]
    #[must_use]
    pub const fn r#where(&self) -> Option<&Where<'a>> {
        self.r#where.as_ref()
    }

    #[inline]
    #[must_use]
    pub const fn group_by(&self) -> Option<&GroupBy<'a>> {
        self.group_by.as_ref()
    }

    #[inline]
    #[must_use]
    pub const fn having(&self) -> Option<&Having<'a>> {
        self.having.as_ref()
    }
}

impl kosame_sql::FmtSql for SelectCore<'_> {
    fn fmt_sql<D>(&self, formatter: &mut kosame_sql::Formatter<D>) -> kosame_sql::Result
    where
        D: kosame_sql::Dialect,
    {
        self.select.fmt_sql(formatter)?;
        self.from.fmt_sql(formatter)?;
        self.r#where.fmt_sql(formatter)?;
        self.group_by.fmt_sql(formatter)?;
        self.having.fmt_sql(formatter)?;
        Ok(())
    }
}
