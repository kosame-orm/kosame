use std::fmt::Write;

use super::Expr;

pub struct Paren<'a> {
    expr: &'a Expr<'a>,
}

impl<'a> Paren<'a> {
    #[inline]
    #[must_use]
    pub const fn new(expr: &'a Expr) -> Self {
        Self { expr }
    }
}

impl kosame_sql::FmtSql for Paren<'_> {
    #[inline]
    fn fmt_sql<D: kosame_sql::Dialect>(
        &self,
        formatter: &mut kosame_sql::Formatter<D>,
    ) -> kosame_sql::Result {
        formatter.write_str("(")?;
        self.expr.fmt_sql(formatter)?;
        formatter.write_str(")")?;
        Ok(())
    }
}
