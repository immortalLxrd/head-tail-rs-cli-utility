use anyhow::{Context, Result};
use clap::Parser;
use cli::cli::{Cli, Command};
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};
use utilities::{head::Head, tail::Tail, traits::execute::Execute};

mod cli;
mod utilities;

fn main() -> Result<()> {
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout);

    let cli = Cli::parse();
    let file = File::open(&cli.path)
        .with_context(|| format!("could not read file \"{}\"", &cli.path.to_string_lossy()))?;
    let reader = BufReader::new(file);
    let file_content: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    match &cli.command {
        Command::Head => {
            let head = Head::new(file_content, cli);
            head.execute(&mut writer)?;
        }
        Command::Tail => {
            let tail = Tail::new(file_content, cli);
            tail.execute(&mut writer)?;
        }
    }

    Ok(())
}
