pub fn reflow_text(input: &str) -> String {
    let has_final_newline = input.ends_with('\n');
    let body = input.strip_suffix('\n').unwrap_or(input);
    let mut out = Vec::new();
    let mut paragraph = Paragraph::default();
    let mut fence: Option<Fence> = None;

    for raw_line in body.split('\n') {
        let line = raw_line.strip_suffix('\r').unwrap_or(raw_line);

        if let Some(current) = fence {
            flush_paragraph(&mut out, &mut paragraph);
            out.push(line.to_owned());
            if closes_fence(line, current) {
                fence = None;
            }
            continue;
        }

        if let Some(opening) = opens_fence(line) {
            flush_paragraph(&mut out, &mut paragraph);
            out.push(line.to_owned());
            fence = Some(opening);
            continue;
        }

        if let Some(quote_line) = parse_quote_line(line) {
            if is_admonition_marker(line) {
                flush_paragraph(&mut out, &mut paragraph);
                out.push(line.to_owned());
                continue;
            }
            if !paragraph.lines.is_empty() && paragraph.prefix != quote_line.prefix {
                flush_paragraph(&mut out, &mut paragraph);
            }
            paragraph.prefix = quote_line.prefix;
            paragraph.lines.push(normalize_whitespace(quote_line.text));
        } else if let Some(list_item) = parse_list_item(line) {
            flush_paragraph(&mut out, &mut paragraph);
            paragraph.prefix = list_item.prefix;
            paragraph.lines.push(normalize_whitespace(list_item.text));
        } else if line.trim().is_empty() {
            flush_paragraph(&mut out, &mut paragraph);
            out.push(String::new());
        } else if is_reflowable(line) {
            paragraph.lines.push(normalize_whitespace(line));
        } else {
            flush_paragraph(&mut out, &mut paragraph);
            out.push(line.to_owned());
        }
    }

    flush_paragraph(&mut out, &mut paragraph);

    let mut result = out.join("\n");
    if has_final_newline {
        result.push('\n');
    }
    result
}

pub fn same_content(before: &str, after: &str) -> bool {
    semantic_chars(before).eq(semantic_chars(after))
}

#[derive(Default)]
struct Paragraph {
    prefix: String,
    lines: Vec<String>,
}

fn flush_paragraph(out: &mut Vec<String>, paragraph: &mut Paragraph) {
    if paragraph.lines.is_empty() {
        return;
    }

    out.push(format!("{}{}", paragraph.prefix, paragraph.lines.join(" ")));
    paragraph.prefix.clear();
    paragraph.lines.clear();
}

fn is_reflowable(line: &str) -> bool {
    let trimmed = line.trim_start();

    if line.starts_with("    ") || line.starts_with('\t') {
        return false;
    }

    !is_markdown_block_line(trimmed)
}

fn is_markdown_block_line(trimmed: &str) -> bool {
    trimmed.starts_with('#')
        || trimmed.starts_with('|')
        || trimmed.starts_with('<')
        || trimmed.starts_with("---")
        || trimmed.starts_with("***")
        || trimmed.starts_with("___")
        || is_list_item(trimmed)
}

struct QuoteLine<'a> {
    prefix: String,
    text: &'a str,
}

fn parse_quote_line(line: &str) -> Option<QuoteLine<'_>> {
    let trimmed = line.trim_start();
    let indent_len = line.len().checked_sub(trimmed.len())?;

    if !trimmed.starts_with('>') {
        return None;
    }

    let marker_end = indent_len.checked_add('>'.len_utf8())?;
    let rest = &line[marker_end..];
    let spaces_len = rest
        .char_indices()
        .take_while(|(_, ch)| ch.is_whitespace())
        .filter_map(|(idx, ch)| idx.checked_add(ch.len_utf8()))
        .last()
        .unwrap_or(0);
    let prefix_end = marker_end.checked_add(spaces_len)?;

    Some(QuoteLine {
        prefix: line[..prefix_end].to_owned(),
        text: &line[prefix_end..],
    })
}

fn is_admonition_marker(line: &str) -> bool {
    let trimmed = line.trim_start();
    let rest = trimmed.strip_prefix('>').unwrap_or(trimmed);
    let spaces = rest.chars().take_while(|ch| *ch == ' ').count();

    if spaces > 4 {
        return false;
    }

    matches!(
        rest[spaces..].trim_end(),
        "[!NOTE]" | "[!TIP]" | "[!IMPORTANT]" | "[!WARNING]" | "[!CAUTION]"
    )
}

struct ListItem<'a> {
    prefix: String,
    text: &'a str,
}

fn parse_list_item(line: &str) -> Option<ListItem<'_>> {
    let trimmed = line.trim_start();
    let indent_len = line.len().checked_sub(trimmed.len())?;
    let marker_len = list_marker_len(trimmed)?;
    let marker_end = indent_len.checked_add(marker_len)?;
    let rest = &line[marker_end..];
    let spaces_len = rest
        .char_indices()
        .take_while(|(_, ch)| ch.is_whitespace())
        .filter_map(|(idx, ch)| idx.checked_add(ch.len_utf8()))
        .last()
        .unwrap_or(0);
    let prefix_end = marker_end.checked_add(spaces_len)?;

    Some(ListItem {
        prefix: line[..prefix_end].to_owned(),
        text: &line[prefix_end..],
    })
}

fn is_list_item(trimmed: &str) -> bool {
    list_marker_len(trimmed).is_some()
}

fn list_marker_len(trimmed: &str) -> Option<usize> {
    let first = trimmed.chars().next()?;

    if matches!(first, '-' | '*' | '+') {
        return trimmed
            .chars()
            .nth(1)
            .is_some_and(char::is_whitespace)
            .then_some(first.len_utf8());
    }

    let (number, rest) = trimmed.split_once('.')?;

    if number.is_empty()
        || !number.chars().all(|ch| ch.is_ascii_digit())
        || !rest.starts_with(char::is_whitespace)
    {
        return None;
    }

    number.len().checked_add('.'.len_utf8())
}

#[derive(Clone, Copy)]
struct Fence {
    marker: char,
    len: usize,
}

fn opens_fence(line: &str) -> Option<Fence> {
    let trimmed = line.trim_start();
    let marker = trimmed.chars().next()?;

    if marker != '`' && marker != '~' {
        return None;
    }

    let len = trimmed.chars().take_while(|ch| *ch == marker).count();
    (len >= 3).then_some(Fence { marker, len })
}

fn closes_fence(line: &str, fence: Fence) -> bool {
    let trimmed = line.trim_start();
    trimmed.chars().take_while(|ch| *ch == fence.marker).count() >= fence.len
}

fn normalize_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn semantic_chars(text: &str) -> impl Iterator<Item = char> + '_ {
    text.lines()
        .map(|line| {
            let line = line.strip_suffix('\r').unwrap_or(line);
            parse_quote_line(line).map_or(line, |quote_line| quote_line.text)
        })
        .flat_map(str::chars)
        .filter(|ch| !ch.is_whitespace())
}

#[cfg(test)]
mod tests {
    use super::{reflow_text, same_content};

    #[test]
    fn reflows_hard_wrapped_plain_prose() {
        let input =
            "This is a paragraph\nthat was wrapped hard\nbefore it reached the preferred width.\n";

        assert_eq!(
            reflow_text(input),
            "This is a paragraph that was wrapped hard before it reached the preferred width.\n"
        );
    }

    #[test]
    fn normalizes_internal_whitespace_runs_in_reflowed_prose() {
        let input = "End of sentence.  Start of next.\n";

        assert_eq!(reflow_text(input), "End of sentence. Start of next.\n");
    }

    #[test]
    fn unwraps_arbitrary_hard_wrap_widths() {
        let words = [
            "alpha", "bravo", "charlie", "delta", "echo", "foxtrot", "golf", "hotel", "india",
            "juliet", "kilo", "lima", "mike", "november", "oscar", "papa", "quebec", "romeo",
            "sierra", "tango", "uniform", "victor", "whiskey", "xray", "yankee", "zulu",
        ];
        let paragraph = words.join(" ");

        for width in [70, 80, 90, 100] {
            let input = hard_wrap(&paragraph, width).join("\n") + "\n";
            let output = reflow_text(&input);

            assert_eq!(output, format!("{paragraph}\n"), "width {width}");
            assert!(same_content(&input, &output));
        }
    }

    #[test]
    fn keeps_markdown_fenced_code_unchanged() {
        let input = "\
Intro line that should join
with the next intro line.

```rust
fn main() {
    println!(\"do not touch this\");
}
```
";

        let output = reflow_text(input);

        assert!(output.contains("Intro line that should join with the next intro line."));
        assert!(output.contains("fn main() {\n    println!(\"do not touch this\");\n}"));
        assert!(same_content(input, &output));
    }

    #[test]
    fn keeps_indented_code_unchanged() {
        let input = "Before\n\n    let x = 1;\n    let y = 2;\n";

        assert_eq!(reflow_text(input), input);
    }

    #[test]
    fn does_not_reflow_markdown_block_lines() {
        let input = "# Heading\n\n- First item\n- Second item\n\n| table |\n";

        assert_eq!(reflow_text(input), input);
    }

    #[test]
    fn reflows_hard_wrapped_blockquotes() {
        let input = "\
# Example text

> [!NOTE]
> line is here 1
> line is here 2
> line is here 3
> line is here 4
> line is here 5

## Overview

Example paragraph.
";

        let output = reflow_text(input);

        assert_eq!(
            output,
            "\
# Example text

> [!NOTE]
> line is here 1 line is here 2 line is here 3 line is here 4 line is here 5

## Overview

Example paragraph.
"
        );
        assert!(same_content(input, &output));
    }

    #[test]
    fn preserves_github_admonition_markers_with_allowed_spacing() {
        let input = "\
> [!TIP]
> first body line
> second body line

>    [!WARNING]
> another body line
> final body line
";

        assert_eq!(
            reflow_text(input),
            "\
> [!TIP]
> first body line second body line

>    [!WARNING]
> another body line final body line
"
        );
    }

    #[test]
    fn reflows_blockquote_lines_that_only_look_like_over_indented_admonitions() {
        let input = ">     [!NOTE]\n>     body line\n";

        assert_eq!(reflow_text(input), ">     [!NOTE] body line\n");
    }

    #[test]
    fn reflows_hard_wrapped_list_items() {
        let input = "\
- **Current state and history are separate concerns.** A live head (current state, the
clean scan/diff baseline) plus append-only per-file history (the version record).

- **Deletion detection is inference-from-absence, always.** A path present in the
baseline but absent from a rescan is deleted.
";

        assert_eq!(
            reflow_text(input),
            "\
- **Current state and history are separate concerns.** A live head (current state, the clean scan/diff baseline) plus append-only per-file history (the version record).

- **Deletion detection is inference-from-absence, always.** A path present in the baseline but absent from a rescan is deleted.
"
        );
    }

    #[test]
    fn preserves_list_marker_prefixes_when_reflowing() {
        let input = "  1. First item\ncontinues here\n  * Second item\ncontinues too\n";

        assert_eq!(
            reflow_text(input),
            "  1. First item continues here\n  * Second item continues too\n"
        );
    }

    fn hard_wrap(text: &str, width: usize) -> Vec<String> {
        let mut lines = Vec::new();
        let mut current = String::new();

        for word in text.split_whitespace() {
            let next_len = if current.is_empty() {
                word.len()
            } else {
                current.len().saturating_add(1).saturating_add(word.len())
            };

            if !current.is_empty() && next_len > width {
                lines.push(current);
                current = String::new();
            }

            if !current.is_empty() {
                current.push(' ');
            }
            current.push_str(word);
        }

        if !current.is_empty() {
            lines.push(current);
        }

        lines
    }
}
