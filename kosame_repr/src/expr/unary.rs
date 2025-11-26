use std::fmt::Write;

use super::Expr;

pub struct Unary<'a> {
    op: UnaryOp,
    operand: &'a Expr<'a>,
}

impl<'a> Unary<'a> {
    #[inline]
    #[must_use]
    pub const fn new(op: UnaryOp, operand: &'a Expr<'a>) -> Self {
        Self { op, operand }
    }
}

impl kosame_sql::FmtSql for Unary<'_> {
    #[inline]
    fn fmt_sql<D: kosame_sql::Dialect>(
        &self,
        formatter: &mut kosame_sql::Formatter<D>,
    ) -> kosame_sql::Result {
        match self.op.position() {
            Position::Prefix => {
                self.op.fmt_sql(formatter)?;
                self.operand.fmt_sql(formatter)?;
            }
            Position::Postfix => {
                self.operand.fmt_sql(formatter)?;
                self.op.fmt_sql(formatter)?;
            }
        }
        Ok(())
    }
}

pub enum UnaryOp {
    Not,
}

pub enum Position {
    Prefix,
    Postfix,
}

impl UnaryOp {
    #[inline]
    #[must_use]
    pub fn position(&self) -> Position {
        match self {
            Self::Not => Position::Prefix,
        }
    }
}

impl kosame_sql::FmtSql for UnaryOp {
    #[inline]
    fn fmt_sql<D: kosame_sql::Dialect>(
        &self,
        formatter: &mut kosame_sql::Formatter<D>,
    ) -> kosame_sql::Result {
        match self {
            Self::Not => formatter.write_str("not "),
        }
    }
}
