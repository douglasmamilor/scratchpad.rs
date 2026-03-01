use crate::text::bitmap::font::GlyphInstance;

use super::BitmapFont;

pub(crate) fn layout_text(
    font: &BitmapFont,
    text: &str,
    start_x: f32,
    start_y: f32,
) -> Vec<GlyphInstance> {
    let mut instances: Vec<GlyphInstance> = Vec::new();
    let (mut cursor_x, mut cursor_y) = (start_x, start_y);
    let mut prev_char: Option<char> = None;

    for ch in text.chars() {
        if ch == '\n' {
            cursor_x = start_x;
            cursor_y = font.line_height() as f32;
            prev_char = None;
            continue;
        }

        let glyph = font
            .glyph_metrics(ch)
            .unwrap_or_else(|| font.glyph_metrics('?').unwrap());

        if let Some(prev) = prev_char {
            cursor_x += font.kerning_amount(prev, ch) as f32;
        }

        let uv_rect = font.uv_rect(ch).unwrap_or((0.0, 0.0, 1.0, 1.0));
        let size = (glyph.width() as usize, glyph.height() as usize);
        instances.push(GlyphInstance::new(ch, cursor_x, cursor_y, uv_rect, size));

        cursor_x += glyph.x_advance() as f32;
        prev_char = Some(ch);
    }

    instances
}

pub(crate) fn measure_text_multiline(
    font: &BitmapFont,
    text: &str,
    max_width: usize,
) -> (usize, usize) {
    let lines = word_wrap(font, text, max_width);

    let mut max_width = 0;

    for line in &lines {
        let (line_width, _) = measure_text(font, line.as_str());
        max_width = max_width.max(line_width);
    }

    (max_width, font.line_height() * lines.len())
}

pub(crate) fn measure_text(font: &BitmapFont, text: &str) -> (usize, usize) {
    let mut width = 0i32;
    let mut prev_char: Option<char> = None;

    for ch in text.chars() {
        let Some(glyph) = font.glyph_metrics(ch) else {
            continue;
        };

        if let Some(prev_char) = prev_char {
            width += font.kerning_amount(prev_char, ch);
        }

        width += glyph.x_advance();
        prev_char = Some(ch);
    }

    (width.max(0) as usize, font.line_height())
}

pub(crate) fn word_wrap(font: &BitmapFont, text: &str, max_width: usize) -> Vec<String> {
    let max_width = max_width.max(1);
    let space_width = measure_text(font, " ").0 as isize;

    let mut lines: Vec<String> = Vec::new();
    let mut current_line = String::new();
    let mut current_width: isize = 0;

    for word in text.split_whitespace() {
        let word_width = measure_text(font, word).0 as isize;

        if current_line.is_empty() {
            current_line.push_str(word);
            current_width = word_width;
            continue;
        }

        let candidate_w = current_width
            .saturating_add(space_width)
            .saturating_add(word_width);

        if candidate_w as usize <= max_width {
            current_line.push(' ');
            current_line.push_str(word);
            current_width = candidate_w;
        } else {
            lines.push(std::mem::take(&mut current_line));
            current_line.push_str(word);
            current_width = word_width;
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}
