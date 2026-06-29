# reflowtext

[![License](https://img.shields.io/badge/license-Apache%202.0-blue)](https://opensource.org/licenses/Apache-2.0)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue)](https://opensource.org/licenses/MIT)

`reflowtext` is a small Rust CLI for removing hard line wraps from prose in plain text and Markdown files.

It edits files in place and leaves Markdown code blocks, lists, headings, tables, quotes, and indented code unchanged.

```sh
reflowtext <file> [file ...]
reflowtext < input.txt
```

File arguments are edited in place. Stdin is written to stdout. Before writing files, it checks that only whitespace changed.

### License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
