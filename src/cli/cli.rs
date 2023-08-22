use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(value_enum)]
    pub command: Command,

    #[arg(short = 'n', long)]
    pub lines: Option<String>,

    pub path: PathBuf,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum Command {
    Head,
    Tail,
}
