pub fn reflow_text(input: &str) -> String {
    let has_final_newline = input.ends_with('\n');
    let body = input.strip_suffix('\n').unwrap_or(input);
    let mut out = Vec::new();
    let mut paragraph = Vec::new();
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

        if line.trim().is_empty() {
            flush_paragraph(&mut out, &mut paragraph);
            out.push(String::new());
        } else if is_reflowable(line) {
            paragraph.push(line.trim().to_owned());
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
    without_whitespace(before) == without_whitespace(after)
}

fn flush_paragraph(out: &mut Vec<String>, paragraph: &mut Vec<String>) {
    if paragraph.is_empty() {
        return;
    }

    out.push(paragraph.join(" "));
    paragraph.clear();
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
        || trimmed.starts_with('>')
        || trimmed.starts_with('|')
        || trimmed.starts_with('<')
        || trimmed.starts_with("---")
        || trimmed.starts_with("***")
        || trimmed.starts_with("___")
        || is_list_item(trimmed)
}

fn is_list_item(trimmed: &str) -> bool {
    let Some(first) = trimmed.chars().next() else {
        return false;
    };

    if matches!(first, '-' | '*' | '+') {
        return trimmed
            .chars()
            .nth(1)
            .is_some_and(|second| second.is_whitespace());
    }

    let Some((number, rest)) = trimmed.split_once('.') else {
        return false;
    };

    !number.is_empty()
        && number.chars().all(|ch| ch.is_ascii_digit())
        && rest.starts_with(char::is_whitespace)
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

fn without_whitespace(text: &str) -> String {
    text.chars().filter(|ch| !ch.is_whitespace()).collect()
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
        let input = "# Heading\n\n- First item\n- Second item\n\n> quoted\n";

        assert_eq!(reflow_text(input), input);
    }

    fn hard_wrap(text: &str, width: usize) -> Vec<String> {
        let mut lines = Vec::new();
        let mut current = String::new();

        for word in text.split_whitespace() {
            let next_len = if current.is_empty() {
                word.len()
            } else {
                current.len() + 1 + word.len()
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
