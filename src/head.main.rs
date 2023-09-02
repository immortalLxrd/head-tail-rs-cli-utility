use anyhow::{Context, Result};
use clap::Parser;
use cli::cli::Cli;
use std::{
    fs::File,
    io::{self, BufReader},
};
use utilities::{head::Head, traits::execute::Execute};

mod cli;
mod utilities;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let file = File::open(&cli.path)
        .with_context(|| format!("could not read file \"{}\"", &cli.path.to_string_lossy()))?;
    let reader = BufReader::new(file);

    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout);

    let mut head = Head::new(reader, cli);
    head.execute(&mut writer)?;

    Ok(())
}
