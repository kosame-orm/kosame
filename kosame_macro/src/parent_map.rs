use std::{cell::RefCell, collections::HashMap, sync::atomic::Ordering};

use crate::{
    clause::{FromItem, WithItem},
    command::Command,
    expr::ColumnRef,
    visitor::Visitor,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Id(u32);

impl Id {
    pub fn new() -> Self {
        static AUTO_INCREMENT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let increment = AUTO_INCREMENT.fetch_add(1, Ordering::Relaxed);
        Self(increment)
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

thread_local! {
    /// Usage of this field is unsafe, see comment below.
    static PARENT_MAP: RefCell<Option<&'static ParentMap<'static>>> = const { RefCell::new(None) };
}

#[derive(Default)]
pub struct ParentMap<'a> {
    map: HashMap<Id, Node<'a>>,
}

impl<'a> ParentMap<'a> {
    pub fn scope(&self, f: impl FnOnce()) {
        PARENT_MAP.with_borrow_mut(|option| {
            if option.is_some() {
                panic!("nested parent map scopes are not allowed");
            }

            *option = Some(unsafe {
                // Safety: The only access point for this parent map is the `with` method called
                // within the stack frame of `f`. The `with` method reverts the static lifetime to
                // a locally scoped one again.
                std::mem::transmute::<&ParentMap<'a>, &'static ParentMap<'static>>(self)
            });
        });
        f();
        PARENT_MAP.with_borrow_mut(|option| *option = None);
    }

    pub fn with<R>(f: impl FnOnce(&ParentMap<'_>) -> R) -> R {
        PARENT_MAP.with_borrow(|parent_map| match parent_map {
            Some(parent_map) => f(parent_map),
            None => panic!("called `ParentMap::with` outside parent map scope"),
        })
    }

    pub fn parent<N>(&self, node: N) -> Option<&Node<'a>>
    where
        N: Into<Node<'a>>,
    {
        self.map.get(node.into().id())
    }

    pub fn seek_parent<N, P>(&self, node: N) -> Option<&'a P>
    where
        P: 'a,
        N: Into<Node<'a>>,
        for<'b> &'a P: TryFrom<&'b Node<'a>>,
    {
        let mut node = &node.into();
        while let Some(parent) = self.map.get(node.id()) {
            if let Ok(parent) = <&'a P>::try_from(parent) {
                return Some(parent);
            }
            node = parent;
        }
        None
    }
}

#[derive(Clone)]
pub enum Node<'a> {
    Command(&'a Command),
    FromItem(&'a FromItem),
    WithItem(&'a WithItem),
    ColumnRef(&'a ColumnRef),
}

impl Node<'_> {
    fn id(&self) -> &Id {
        match self {
            Self::Command(inner) => &inner.id,
            Self::FromItem(inner) => inner.id(),
            Self::WithItem(inner) => &inner.id,
            Self::ColumnRef(inner) => &inner.id,
        }
    }
}

macro_rules! impl_node_variant {
    ($type:ident) => {
        impl<'a> From<&'a $type> for Node<'a> {
            fn from(v: &'a $type) -> Self {
                Self::$type(v)
            }
        }

        impl<'a> TryFrom<&Node<'a>> for &'a $type {
            type Error = ();
            fn try_from(value: &Node<'a>) -> Result<Self, Self::Error> {
                match value {
                    Node::$type(inner) => Ok(inner),
                    _ => Err(()),
                }
            }
        }
    };
}

impl_node_variant!(Command);
impl_node_variant!(FromItem);
impl_node_variant!(WithItem);
impl_node_variant!(ColumnRef);

#[derive(Default)]
pub struct ParentMapBuilder<'a> {
    parent_map: ParentMap<'a>,
    stack: Vec<Node<'a>>,
}

impl<'a> ParentMapBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build(self) -> ParentMap<'a> {
        self.parent_map
    }
}

impl<'a> Visitor<'a> for ParentMapBuilder<'a> {
    fn visit_parent_node(&mut self, node: Node<'a>) {
        if let Some(parent) = self.stack.last()
            && self
                .parent_map
                .map
                .insert(*node.id(), parent.clone())
                .is_some()
        {
            panic!("node has multiple parents");
        }
        self.stack.push(node);
    }

    fn end_parent_node(&mut self) {
        self.stack.pop();
    }
}
