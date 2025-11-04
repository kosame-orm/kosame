use syn::Path;

use crate::{expr::BindParam, parent_map::Node};

pub trait Visitor<'a> {
    fn visit_bind_param(&mut self, _bind_param: &'a BindParam) {}

    fn visit_table_ref(&mut self, _table_ref: &'a Path) {}

    fn visit_parent_node(&mut self, _node: Node<'a>) {}
    fn end_parent_node(&mut self) {}
}

impl<'a, T> Visitor<'a> for T
where
    T: FnMut(&'a Path),
{
    fn visit_table_ref(&mut self, table_ref: &'a Path) {
        self(table_ref)
    }
}
