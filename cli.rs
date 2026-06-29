use anyhow::{Result, bail};
use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Debug, Eq, PartialEq)]
pub struct Args {
    pub paths: Vec<PathBuf>,
}

impl Args {
    pub fn parse<I>(args: I) -> Result<Self>
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
                bail!("unknown option: {}", arg.to_string_lossy());
            }

            let path = PathBuf::from(arg);
            if !path.is_file() {
                bail!("not an existing file: {}", path.display());
            }
            paths.push(path);
        }

        if paths.is_empty() {
            bail!("usage: reflowtext <file> [file ...]");
        }

        Ok(Self { paths })
    }
}

fn print_help() {
    println!(
        "\
Usage: reflowtext <file> [file ...]

Reflow hard-line wrapped prose in text or markdown files in place.

Source code blocks in markdown are left unchanged."
    );
}

#[cfg(test)]
mod tests {
    use super::Args;
    use std::ffi::OsString;

    #[test]
    fn rejects_empty_args() {
        let err = Args::parse([OsString::from("reflowtext")]).unwrap_err();
        assert!(err.to_string().contains("usage:"));
    }
}
