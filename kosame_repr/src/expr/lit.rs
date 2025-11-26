use std::fmt::Write;

pub enum Lit {
    Int(i64),
    Float(f64),
    Str(&'static str),
    Bool(bool),
    Null,
}

impl kosame_sql::FmtSql for Lit {
    #[inline]
    fn fmt_sql<D: kosame_sql::Dialect>(
        &self,
        formatter: &mut kosame_sql::Formatter<D>,
    ) -> kosame_sql::Result {
        match self {
            Self::Int(inner) => write!(formatter, "{inner}"),
            Self::Float(inner) => write!(formatter, "{inner}"),
            Self::Str(inner) => write!(formatter, "'{}'", inner.replace('\'', "''")),
            Self::Bool(inner) => write!(formatter, "{inner}"),
            Self::Null => formatter.write_str("null"),
        }
    }
}
