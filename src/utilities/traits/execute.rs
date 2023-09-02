use anyhow::Result;
use std::io::{BufWriter, Stdout};

pub trait Execute {
    fn execute(&mut self, writer: &mut BufWriter<Stdout>) -> Result<()>;
}
