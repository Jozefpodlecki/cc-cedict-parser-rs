use anyhow::{Context, Result};

mod basic;
mod rich;

use cc_cedict_parser_rs::*;

fn main() -> Result<()> {
    
    basic::example()?;
    // rich::example()?;

    Ok(())
}