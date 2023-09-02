use super::traits::execute::Execute;
use crate::cli::cli::Cli;
use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Read, Stdout, Write},
};

#[derive(Debug)]
pub struct Head {
    reader: BufReader<File>,
    cli: Cli,
}

impl Head {
    pub fn new(reader: BufReader<File>, cli: Cli) -> Self {
        Head { reader, cli }
    }

    fn byte_writer(&mut self, mut byte_count: usize, writer: &mut BufWriter<Stdout>) -> Result<()> {
        let mut line = String::new();

        while byte_count > 0 {
            let byte = match self.reader.by_ref().bytes().next() {
                Some(value) => value?,
                None => break,
            };

            if byte == b'\n' {
                writeln!(writer, "{}", line)?;
                line = String::new();
            } else {
                let char = char::from(byte);
                line.push(char);
            }
            byte_count -= 1;
        }

        if line.len() > 0 {
            line.push('⏎');
            writeln!(writer, "{}", line)?;
        }

        Ok(())
    }

    fn line_writer(&mut self, mut line_count: usize, writer: &mut BufWriter<Stdout>) -> Result<()> {
        while line_count > 0 {
            let line = match self.reader.by_ref().lines().next() {
                Some(value) => value,
                None => return Ok(()),
            };
            writeln!(writer, "{}", line?)?;
            line_count -= 1;
        }

        Ok(())
    }

    fn byte_form(&mut self, byte_count: String, writer: &mut BufWriter<Stdout>) -> Result<()> {
        let byte_count_parsed = byte_count.parse::<usize>().unwrap();
        self.byte_writer(byte_count_parsed, writer)?;

        Ok(())
    }

    fn line_form(&mut self, line_count: String, writer: &mut BufWriter<Stdout>) -> Result<()> {
        let line_count_parsed = line_count.parse::<usize>().unwrap();
        self.line_writer(line_count_parsed, writer)?;

        Ok(())
    }
}

impl Execute for Head {
    fn execute(&mut self, writer: &mut BufWriter<Stdout>) -> Result<()> {
        if self.cli.verbose && !self.cli.silent {
            let path = self.cli.path.to_str().unwrap();
            writeln!(writer, "==> {} <==", path)?;
        }

        match &self.cli {
            Cli {
                bytes: Some(byte_count),
                ..
            } => {
                self.byte_form(byte_count.clone(), writer)?;
            }
            Cli {
                lines: Some(line_count),
                ..
            } => {
                self.line_form(line_count.clone(), writer)?;
            }
            Cli { .. } => self.line_form(String::from("10"), writer)?,
        }

        Ok(())
    }
}
