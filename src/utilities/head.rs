use super::traits::execute::Execute;
use crate::cli::cli::Cli;
use anyhow::Result;
use std::io::{BufWriter, Stdout, Write};

#[derive(Debug)]
pub struct Head {
    file_content: Vec<String>,
    cli: Cli,
}

impl Head {
    pub fn new(file_content: Vec<String>, cli: Cli) -> Head {
        Head { file_content, cli }
    }
}

impl Execute for Head {
    fn execute(&self, writer: &mut BufWriter<Stdout>) -> Result<()> {
        let file_content: Vec<String> = match &self.cli.lines {
            Some(lines) => {
                let lines_count_parsed = lines.parse::<usize>()?;
                (&self.file_content[..lines_count_parsed]).to_vec()
            }
            None => (&self.file_content[..10]).to_vec(),
        };

        for line in file_content {
            writeln!(writer, "{}", line)?;
        }

        Ok(())
    }
}
