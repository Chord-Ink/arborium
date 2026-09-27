//! Render Arborium syntax highlighting as Ratatui lines.
//!
//! Enable languages on your `arborium` dependency. The returned lines own their
//! text and can be passed directly to `ratatui::widgets::Paragraph`.

use arborium::theme::{Style as ThemeStyle, Theme};
use arborium::{Error, Highlighter};
use arborium_highlight::{Span, spans_to_flat_tokens};
use arborium_theme::highlights;
use ratatui_core::{
    style::{Color, Modifier, Style},
    text::{Line, Span as TextSpan},
};

/// Highlight source into owned Ratatui lines, preserving blank lines and Unicode.
pub fn highlight(
    highlighter: &mut Highlighter,
    theme: &Theme,
    language: &str,
    source: &str,
) -> Result<Vec<Line<'static>>, Error> {
    let spans = highlighter.highlight_spans(language, source)?;
    Ok(spans_to_lines(source, spans, theme))
}

/// Convert raw highlight spans, resolving nesting and overlap as the HTML renderer does.
pub fn spans_to_lines(source: &str, spans: Vec<Span>, theme: &Theme) -> Vec<Line<'static>> {
    let mut base = Style::default();
    if let Some(c) = theme.foreground {
        base = base.fg(Color::Rgb(c.r, c.g, c.b));
    }
    if let Some(c) = theme.background {
        base = base.bg(Color::Rgb(c.r, c.g, c.b));
    }
    let mut lines = vec![Line::default().style(base)];
    let mut position = 0;
    for token in spans_to_flat_tokens(source, spans) {
        let start = token.start as usize;
        let end = token.end as usize;
        append(&mut lines, &source[position..start], base, base);
        let style = highlights::tag_to_name(token.tag)
            .and_then(|name| highlights::slot_to_highlight_index(highlights::capture_to_slot(name)))
            .and_then(|index| theme.style(index))
            .map_or(base, |style| base.patch(to_style(style)));
        append(&mut lines, &source[start..end], style, base);
        position = end;
    }
    append(&mut lines, &source[position..], base, base);
    lines
}

fn append(lines: &mut Vec<Line<'static>>, text: &str, style: Style, base: Style) {
    for (index, part) in text.split('\n').enumerate() {
        if index > 0 {
            lines.push(Line::default().style(base));
        }
        if !part.is_empty() {
            lines
                .last_mut()
                .unwrap()
                .spans
                .push(TextSpan::styled(part.to_owned(), style));
        }
    }
}

fn to_style(style: &ThemeStyle) -> Style {
    let mut result = Style::default();
    if let Some(c) = style.fg {
        result = result.fg(Color::Rgb(c.r, c.g, c.b));
    }
    if let Some(c) = style.bg {
        result = result.bg(Color::Rgb(c.r, c.g, c.b));
    }
    for (enabled, modifier) in [
        (style.modifiers.bold, Modifier::BOLD),
        (style.modifiers.italic, Modifier::ITALIC),
        (style.modifiers.underline, Modifier::UNDERLINED),
        (style.modifiers.strikethrough, Modifier::CROSSED_OUT),
    ] {
        if enabled {
            result = result.add_modifier(modifier);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_text_and_blank_lines() {
        let source = "fn main() { /* café 🎉 */ }\n\n";
        let lines = highlight(
            &mut Highlighter::new(),
            &arborium::theme::builtin::catppuccin_mocha(),
            "rust",
            source,
        )
        .unwrap();
        let text = lines
            .iter()
            .map(|line| {
                line.spans
                    .iter()
                    .map(|s| s.content.as_ref())
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");
        assert_eq!(text, source);
        assert_eq!(lines.len(), 3);
    }

    #[test]
    fn nested_spans_do_not_duplicate_text() {
        let theme = Theme::new("test");
        let lines = spans_to_lines(
            "abcd",
            vec![
                Span {
                    start: 0,
                    end: 4,
                    capture: "string".into(),
                    pattern_index: 0,
                },
                Span {
                    start: 1,
                    end: 3,
                    capture: "keyword".into(),
                    pattern_index: 1,
                },
            ],
            &theme,
        );
        assert_eq!(
            lines[0]
                .spans
                .iter()
                .map(|s| s.content.as_ref())
                .collect::<String>(),
            "abcd"
        );
    }

    #[test]
    fn converts_all_style_modifiers() {
        let style = ThemeStyle::new()
            .fg(arborium::theme::Color::new(1, 2, 3))
            .bold()
            .italic()
            .underline()
            .strikethrough();
        let style = to_style(&style);
        assert_eq!(style.fg, Some(Color::Rgb(1, 2, 3)));
        assert!(style.add_modifier.contains(
            Modifier::BOLD | Modifier::ITALIC | Modifier::UNDERLINED | Modifier::CROSSED_OUT
        ));
    }
}
