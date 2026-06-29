# reflowtext

`reflowtext` is a small Rust CLI for removing hard line wraps from prose in plain text and Markdown files.

It edits files in place and leaves Markdown code blocks, lists, headings, tables, quotes, and indented code unchanged.

```sh
reflowtext <file> [file ...]
reflowtext < input.txt
```

File arguments are edited in place. Stdin is written to stdout. Before writing files, it checks that only whitespace changed.

License: MIT OR Apache-2.0.
