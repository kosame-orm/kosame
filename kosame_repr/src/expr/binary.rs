use std::fmt::Write;

use super::Expr;

pub struct Binary<'a> {
    left: &'a Expr<'a>,
    op: BinOp,
    right: &'a Expr<'a>,
}

impl<'a> Binary<'a> {
    #[inline]
    #[must_use]
    pub const fn new(left: &'a Expr<'a>, op: BinOp, right: &'a Expr<'a>) -> Self {
        Self { left, op, right }
    }
}

impl kosame_sql::FmtSql for Binary<'_> {
    #[inline]
    fn fmt_sql<D: kosame_sql::Dialect>(
        &self,
        formatter: &mut kosame_sql::Formatter<D>,
    ) -> std::fmt::Result {
        self.left.fmt_sql(formatter)?;
        self.op.fmt_sql(formatter)?;
        self.right.fmt_sql(formatter)?;
        Ok(())
    }
}

pub enum BinOp {
    // multiplication, division, modulo
    Multiply,
    Divide,
    Modulo,
    // addition, subtraction
    Add,
    Subtract,
    // comparison operators
    Eq,
    Uneq,
    LessThan,
    GreaterThan,
    LessThanOrEq,
    GreaterThanOrEq,
    // is
    Is,
    IsNot,
    IsDistinctFrom,
    // logical
    And,
    Or,
}

impl kosame_sql::FmtSql for BinOp {
    #[inline]
    fn fmt_sql<D: kosame_sql::Dialect>(
        &self,
        formatter: &mut kosame_sql::Formatter<D>,
    ) -> kosame_sql::Result {
        match self {
            Self::Multiply => formatter.write_str(" * "),
            Self::Divide => formatter.write_str(" / "),
            Self::Modulo => formatter.write_str(" % "),
            Self::Add => formatter.write_str(" + "),
            Self::Subtract => formatter.write_str(" - "),
            Self::Eq => formatter.write_str(" = "),
            Self::Uneq => formatter.write_str(" <> "),
            Self::LessThan => formatter.write_str(" < "),
            Self::GreaterThan => formatter.write_str(" > "),
            Self::LessThanOrEq => formatter.write_str(" <= "),
            Self::GreaterThanOrEq => formatter.write_str(" >= "),
            Self::Is => formatter.write_str(" is "),
            Self::IsNot => formatter.write_str(" is not "),
            Self::IsDistinctFrom => formatter.write_str(" is distinct from "),
            Self::And => formatter.write_str(" and "),
            Self::Or => formatter.write_str(" or "),
        }
    }
}
