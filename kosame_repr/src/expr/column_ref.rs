use std::fmt::Write;

pub struct ColumnRef<'a> {
    correlation: Option<&'a str>,
    column: &'a str,
}

impl<'a> ColumnRef<'a> {
    #[inline]
    #[must_use]
    pub const fn new(correlation: Option<&'a str>, column: &'a str) -> Self {
        Self {
            correlation,
            column,
        }
    }
}

impl kosame_sql::FmtSql for ColumnRef<'_> {
    #[inline]
    fn fmt_sql<D: kosame_sql::Dialect>(
        &self,
        formatter: &mut kosame_sql::Formatter<D>,
    ) -> kosame_sql::Result {
        if let Some(correlation) = &self.correlation {
            formatter.write_ident(correlation)?;
            formatter.write_str(".")?;
        }
        formatter.write_ident(self.column)?;
        Ok(())
    }
}
