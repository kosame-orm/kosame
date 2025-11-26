use std::fmt::Write;

use super::Expr;

pub struct Cast<'a> {
    value: &'a Expr<'a>,
    data_type: &'a str,
}

impl<'a> Cast<'a> {
    #[inline]
    #[must_use]
    pub const fn new(value: &'a Expr, data_type: &'a str) -> Self {
        Self { value, data_type }
    }
}

impl kosame_sql::FmtSql for Cast<'_> {
    #[inline]
    fn fmt_sql<D: kosame_sql::Dialect>(
        &self,
        formatter: &mut kosame_sql::Formatter<D>,
    ) -> kosame_sql::Result {
        formatter.write_str("cast(")?;
        self.value.fmt_sql(formatter)?;
        formatter.write_str(" as ")?;
        formatter.write_str(self.data_type)?;
        formatter.write_str(")")?;
        Ok(())
    }
}
