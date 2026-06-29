mod cli;
mod reflow;

use anyhow::{Context, Result, bail};
use std::fs;
use std::io::{self, IsTerminal, Read, Write};

fn main() -> Result<()> {
    let args = cli::Args::parse(std::env::args_os())?;

    if args.paths.is_empty() {
        return reflow_stdin();
    }

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

fn reflow_stdin() -> Result<()> {
    if io::stdin().is_terminal() {
        bail!("usage: reflowtext <file> [file ...] or reflowtext < input.txt");
    }

    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .context("failed to read stdin")?;

    let output = reflow::reflow_text(&input);
    io::stdout()
        .write_all(output.as_bytes())
        .context("failed to write stdout")?;

    Ok(())
}
