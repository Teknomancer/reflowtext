mod cli;
mod reflow;

use std::fs;
use std::io::{self, IsTerminal, Read, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let args = cli::Args::parse(std::env::args_os(), stdin.is_terminal())?;

    if args.paths.is_empty() {
        return reflow_stdin(&stdin);
    }

    for path in args.paths {
        let input = fs::read_to_string(&path).map_err(|err| {
            io::Error::new(
                err.kind(),
                format!("failed to read {}: {err}", path.display()),
            )
        })?;
        let output = reflow::reflow_text(&input);

        if !reflow::same_content(&input, &output) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "refusing to overwrite {} because reflow changed non-whitespace content",
                    path.display()
                ),
            ));
        }

        if output != input {
            fs::write(&path, output).map_err(|err| {
                io::Error::new(
                    err.kind(),
                    format!("failed to write {}: {err}", path.display()),
                )
            })?;
        }
    }

    Ok(())
}

fn reflow_stdin(stdin: &io::Stdin) -> io::Result<()> {
    let mut input = String::new();
    stdin
        .lock()
        .read_to_string(&mut input)
        .map_err(|err| io::Error::new(err.kind(), format!("failed to read stdin: {err}")))?;

    let output = reflow::reflow_text(&input);
    io::stdout()
        .write_all(output.as_bytes())
        .map_err(|err| io::Error::new(err.kind(), format!("failed to write stdout: {err}")))?;

    Ok(())
}
