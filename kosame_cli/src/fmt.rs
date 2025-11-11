use std::io::Read;

use clap::Args;
use syn::spanned::Spanned;

#[derive(Args)]
#[command(version, about = "Format the content of Kosame macro invocations in Rust source files", long_about = None)]
pub struct Fmt {
    #[arg(short, long)]
    file: Option<std::path::PathBuf>,
}

impl Fmt {
    pub fn run(&self) -> anyhow::Result<()> {
        let input = match &self.file {
            Some(file) => std::fs::read_to_string(file)?,
            None => {
                let mut buf = String::new();
                std::io::stdin().read_to_string(&mut buf)?;
                buf
            }
        };
        let mut output = String::new();

        struct Replace {
            start: usize,
            end: usize,
            replacement: String,
        }

        #[derive(Default)]
        struct Visitor {
            indent: usize,
            replacements: Vec<Replace>,
        }
        use syn::visit::Visit;
        impl<'ast> Visit<'ast> for Visitor {
            fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {
                self.indent += 1;
                syn::visit::visit_item_mod(self, node);
                self.indent -= 1;
            }
            fn visit_item_impl(&mut self, node: &'ast syn::ItemImpl) {
                self.indent += 1;
                syn::visit::visit_item_impl(self, node);
                self.indent -= 1;
            }
            fn visit_item_trait(&mut self, node: &'ast syn::ItemTrait) {
                self.indent += 1;
                syn::visit::visit_item_trait(self, node);
                self.indent -= 1;
            }
            fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
                self.indent += 1;
                syn::visit::visit_item_fn(self, node);
                self.indent -= 1;
            }
            fn visit_macro(&mut self, i: &'ast syn::Macro) {
                let name = &i.path.segments.last().expect("paths cannot be empty").ident;
                let span = i.delimiter.span().span();
                match name.to_string().as_ref() {
                    "table" | "pg_table" => {
                        self.replacements.push(Replace {
                            start: span.byte_range().start + 1,
                            end: span.byte_range().end - 1,
                            replacement: "kek".to_string(),
                        });
                    }
                    _ => {}
                }
                println!("{:#?}", i.tokens.span().source_text());
            }
        }

        let file = syn::parse_file(&input)?;
        let mut visitor = Visitor::default();
        visitor.visit_file(&file);

        let mut current_index = 0;
        for replacement in visitor.replacements {
            output.push_str(&input[current_index..replacement.start]);
            output.push_str(&replacement.replacement);
            current_index = replacement.end;
        }

        output.push_str(&input[current_index..]);

        match &self.file {
            Some(file) => std::fs::write(file, output).unwrap(),
            None => print!("{}", output),
        };

        Ok(())
    }
}
