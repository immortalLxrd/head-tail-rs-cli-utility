use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(short = 'c', long)]
    pub bytes: Option<String>,

    #[arg(short = 'n', long)]
    pub lines: Option<String>,

    #[arg(short = 'q', long)]
    pub silent: bool,

    #[arg(short = 'v', long)]
    pub verbose: bool,

    pub path: PathBuf,
}
