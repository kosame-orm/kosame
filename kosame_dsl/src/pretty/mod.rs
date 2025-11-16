mod printer;

pub use printer::*;

pub trait PrettyPrint {
    fn pretty_print(&self, printer: &mut Printer);
}
