use super::traits::execute::Execute;
use crate::cli::cli::Cli;
use anyhow::Result;
use std::io::{BufWriter, Stdout, Write};

#[derive(Debug)]
pub struct Tail {
    file_content: Vec<String>,
    cli: Cli,
}

impl Tail {
    pub fn new(file_content: Vec<String>, cli: Cli) -> Tail {
        Tail { file_content, cli }
    }
}

impl Execute for Tail {
    fn execute(&self, writer: &mut BufWriter<Stdout>) -> Result<()> {
        let file_content: Vec<String> = match &self.cli.lines {
            Some(lines) => {
                let lines_count_parsed = lines.parse::<usize>()?;
                (&self.file_content[(&self.file_content.len() - lines_count_parsed)..]).to_vec()
            }
            None => (&self.file_content[(&self.file_content.len() - 10)..]).to_vec(),
        };

        for line in file_content {
            writeln!(writer, "{}", line)?;
        }

        Ok(())
    }
}
