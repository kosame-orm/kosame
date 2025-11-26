use crate::expr::Expr;

pub struct Column<'a> {
    pub name: &'a str,
    pub data_type: &'a str,
    pub primary_key: bool,
    pub not_null: bool,
    pub default: Option<&'a Expr<'a>>,
}

impl<'a> Column<'a> {
    #[inline]
    #[must_use]
    pub const fn name(&self) -> &'a str {
        self.name
    }

    #[inline]
    #[must_use]
    pub const fn data_type(&self) -> &'a str {
        self.data_type
    }

    #[inline]
    #[must_use]
    pub const fn primary_key(&self) -> bool {
        self.primary_key
    }

    #[inline]
    #[must_use]
    pub const fn not_null(&self) -> bool {
        self.not_null
    }

    #[inline]
    #[must_use]
    pub const fn default(&self) -> Option<&'a Expr<'_>> {
        self.default
    }
}
