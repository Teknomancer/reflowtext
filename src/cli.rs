use std::ffi::OsString;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Eq, PartialEq)]
pub struct Args {
    pub paths: Vec<PathBuf>,
}

impl Args {
    pub fn parse<I>(args: I, stdin_is_terminal: bool) -> io::Result<Self>
    where
        I: IntoIterator<Item = OsString>,
    {
        let mut paths = Vec::new();
        let mut iter = args.into_iter();
        let _program = iter.next();

        for arg in iter {
            if arg == "-h" || arg == "--help" {
                print_help();
                std::process::exit(0);
            }

            if arg.to_string_lossy().starts_with('-') {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("unknown option: {}", arg.to_string_lossy()),
                ));
            }

            let path = PathBuf::from(arg);
            if !path.is_file() {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("not an existing file: {}", path.display()),
                ));
            }
            paths.push(path);
        }

        if paths.is_empty() && stdin_is_terminal {
            print_help();
            std::process::exit(1);
        }

        Ok(Self { paths })
    }
}

fn print_help() {
    println!(
        "\
Usage: reflowtext <file> [file ...]
       reflowtext < input.txt

Remove hard line wraps from prose in text or markdown files in place.

With stdin, writes the reflowed text to stdout. Markdown code blocks are left
unchanged."
    );
}

#[cfg(test)]
mod tests {
    use super::Args;
    use std::ffi::OsString;

    #[test]
    fn accepts_empty_args_for_stdin_mode() -> std::io::Result<()> {
        let args = Args::parse([OsString::from("reflowtext")], false)?;
        assert!(args.paths.is_empty());
        Ok(())
    }
}
