mod printer;
mod text;

pub use printer::*;
pub use text::*;

pub trait PrettyPrint {
    fn pretty_print(&self, printer: &mut Printer);
}
