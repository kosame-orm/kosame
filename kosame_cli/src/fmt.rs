use std::io::Read;

use clap::Args;

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

        const MACROS: [&str; 6] = [
            "table",
            "pg_table",
            "statement",
            "pg_statement",
            "query",
            "pg_query",
        ];

        let mut line = 1;
        let mut column = 1;
        let mut word = String::new();
        let mut stack = Vec::new();
        for (i, c) in input.char_indices() {
            match c {
                '!' => {
                    if MACROS.contains(&word.as_ref()) {
                        println!("{word}");
                    }
                }
                '\n' => {
                    line += 1;
                    column = 1;
                }
                '(' | '{' | '[' => stack.push(c),
                ')' | '}' | ']' => {
                    let Some(opening) = stack.pop() else {
                        anyhow::bail!(
                            "line {line}, column {column}: closing parenthesis found without matching opening parenthesis"
                        );
                    };
                    match (opening, c) {
                        ('(', ')') => {}
                        ('{', '}') => {}
                        ('[', ']') => {}
                        _ => {
                            anyhow::bail!(
                                "line {line}, column {column}: mismatched closing parenthesis"
                            );
                        }
                    }
                }
                _ => {}
            }
            match c.is_alphanumeric() || c == '_' {
                true => word.push(c),
                false => word.clear(),
            }

            output.push(c);
            column += 1;
        }

        if !stack.is_empty() {
            anyhow::bail!("unmatched opening parentheses at the end of the file");
        }

        match &self.file {
            Some(file) => std::fs::write(file, output).unwrap(),
            None => print!("{}", output),
        };

        Ok(())
    }
}
