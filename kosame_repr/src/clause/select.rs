use std::fmt::Write;

use crate::clause::Fields;

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
