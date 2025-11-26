use std::fmt::Write;

pub struct Raw<'a> {
    string: &'a str,
}

impl<'a> Raw<'a> {
    #[inline]
    #[must_use]
    pub const fn new(string: &'a str) -> Self {
        Self { string }
    }
}

impl kosame_sql::FmtSql for Raw<'_> {
    #[inline]
    fn fmt_sql<D: kosame_sql::Dialect>(
        &self,
        formatter: &mut kosame_sql::Formatter<D>,
    ) -> kosame_sql::Result {
        formatter.write_str(self.string)
    }
}
