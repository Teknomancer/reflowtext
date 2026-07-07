# reflowtext

[![Build](https://github.com/Teknomancer/reflowtext/workflows/Build/badge.svg)](https://github.com/Teknomancer/reflowtext/actions?query=workflow%3ABuild)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue)](https://opensource.org/licenses/Apache-2.0)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue)](https://opensource.org/licenses/MIT)

`reflowtext` is a small Rust CLI for removing hard line wraps from prose in plain text and Markdown files.

It edits files in place and leaves Markdown code blocks, headings, tables, and indented code unchanged.
List item and blockquote prose is reflowed while preserving the Markdown marker.
GitHub admonition markers such as `> [!NOTE]` are kept on their own line.
Source code blocks in Markdown are not subject to reflowing.

```sh
reflowtext <file> [file ...]
reflowtext < input.txt
```

File arguments are edited in place. Stdin is written to stdout. Before writing files, it checks that only whitespace changed.

### Example

Given this Markdown:

````markdown
# Notes

This paragraph was copied from a file
where prose had been hard wrapped
at a fixed editor width.

- List items can also be copied from a file
where prose had been hard wrapped.

```text
code blocks
stay exactly as written
```
````

Running `reflowtext notes.md` rewrites only the prose paragraph:

````markdown
# Notes

This paragraph was copied from a file where prose had been hard wrapped at a fixed editor width.

- List items can also be copied from a file where prose had been hard wrapped.

```text
code blocks
stay exactly as written
```
````

### Development

The repo defines Cargo aliases for the regular local workflow:

```sh
cargo build-all
cargo test-all
cargo lint
```

### License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
