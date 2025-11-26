pub struct BindParam<'a> {
    name: &'a str,
    ordinal: u32,
}

impl<'a> BindParam<'a> {
    #[inline]
    #[must_use]
    pub const fn new(name: &'a str, ordinal: u32) -> Self {
        Self { name, ordinal }
    }
}

impl kosame_sql::FmtSql for BindParam<'_> {
    #[inline]
    fn fmt_sql<D: kosame_sql::Dialect>(
        &self,
        formatter: &mut kosame_sql::Formatter<D>,
    ) -> kosame_sql::Result {
        formatter.write_bind_param(self.name, self.ordinal)
    }
}
