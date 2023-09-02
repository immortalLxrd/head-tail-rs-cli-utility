use super::traits::execute::Execute;
use crate::cli::cli::Cli;
use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Stdout, Write},
};

#[derive(Debug)]
pub struct Tail {
    reader: BufReader<File>,
    cli: Cli,
}

impl Tail {
    pub fn new(reader: BufReader<File>, cli: Cli) -> Self {
        Tail { reader, cli }
    }

    fn indent_to_end_by_bytes(&mut self, byte_count_parsed: usize) -> Result<()> {
        match self.reader.seek(SeekFrom::End(-(byte_count_parsed as i64))) {
            Ok(_) => (),
            Err(_) => return Ok(()),
        };

        Ok(())
    }

    fn indent_to_end_by_lines(&mut self, line_count_parsed: usize) -> Result<()> {
        match self.reader.seek(SeekFrom::End(0)) {
            Ok(_) => (),
            Err(_) => return Ok(()),
        };

        let mut current_lines_count = line_count_parsed + 1;

        while current_lines_count > 0 {
            match self.reader.seek(SeekFrom::Current(-1)) {
                Ok(_) => (),
                Err(_) => return Ok(()),
            };

            let byte = self.reader.by_ref().fill_buf()?[0];

            if byte == b'\n' {
                current_lines_count -= 1;
            }
        }

        self.reader.seek(SeekFrom::Current(1))?;

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

    fn byte_form(&mut self, byte_count: String, writer: &mut BufWriter<Stdout>) -> Result<()> {
        let byte_count_parsed = byte_count.parse::<usize>().unwrap();

        self.indent_to_end_by_bytes(byte_count_parsed)?;
        self.byte_writer(byte_count_parsed, writer)?;

        Ok(())
    }

    fn line_form(&mut self, line_count: String, writer: &mut BufWriter<Stdout>) -> Result<()> {
        let line_count_parsed = line_count.parse::<usize>().unwrap();

        self.indent_to_end_by_lines(line_count_parsed)?;
        self.line_writer(line_count_parsed, writer)?;

        Ok(())
    }
}

impl Execute for Tail {
    fn execute(&mut self, buf_writer: &mut BufWriter<Stdout>) -> Result<()> {
        if self.cli.verbose && !self.cli.silent {
            let path = self.cli.path.to_str().unwrap();
            writeln!(buf_writer, "==> {} <==", path)?;
        }

        match &self.cli {
            Cli {
                bytes: Some(byte_count),
                ..
            } => {
                self.byte_form(byte_count.clone(), buf_writer)?;
            }
            Cli {
                lines: Some(line_count),
                ..
            } => {
                self.line_form(line_count.clone(), buf_writer)?;
            }
            Cli { .. } => self.line_form(String::from("10"), buf_writer)?,
        }

        Ok(())
    }
}
