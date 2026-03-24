
use anyhow::{Context, Result};

use cc_cedict_parser_rs::*;

pub fn example() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let file_path = current_dir.join("cedict_ts.u8");
    let reader = LineReader::from_file(&file_path)?;
    
    for line in reader {
        let line = line?;
        let matches = line.matches("abbr");

        if matches.count() > 1 {
            println!("{line}");
        }

        let entry = Entry::new(&line).with_context(|| "Could not parse line")?;
    }


    Ok(())
}