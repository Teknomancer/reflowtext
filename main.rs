mod cli;
mod reflow;

use anyhow::{Context, Result, bail};
use std::fs;

fn main() -> Result<()> {
    let args = cli::Args::parse(std::env::args_os())?;

    for path in args.paths {
        let input = fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        let output = reflow::reflow_text(&input);

        if !reflow::same_content(&input, &output) {
            bail!(
                "refusing to overwrite {} because reflow changed non-whitespace content",
                path.display()
            );
        }

        if output != input {
            fs::write(&path, output)
                .with_context(|| format!("failed to write {}", path.display()))?;
        }
    }

    Ok(())
}
