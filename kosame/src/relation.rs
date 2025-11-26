use std::ops::{Deref, DerefMut};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde-full", derive(serde::Deserialize))]
pub struct One<T>(T);

impl<T> One<T> {
    pub fn new(inner: T) -> Self {
        Self(inner)
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deref for One<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for One<T> {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde-full", derive(serde::Deserialize))]
pub struct ZeroOrOne<T>(Option<T>);

impl<T> ZeroOrOne<T> {
    pub fn new(inner: Option<T>) -> Self {
        Self(inner)
    }

    pub fn into_option(self) -> Option<T> {
        self.0
    }
}

impl<T> Deref for ZeroOrOne<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for ZeroOrOne<T> {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde-full", derive(serde::Deserialize))]
pub struct Many<T>(Vec<T>);

impl<T> Many<T> {
    #[inline]
    #[must_use]
    pub fn new(inner: Vec<T>) -> Self {
        Self(inner)
    }

    #[inline]
    #[must_use]
    pub fn into_vec(self) -> Vec<T> {
        self.0
    }
}

impl<T> Deref for Many<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Many<T> {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}
