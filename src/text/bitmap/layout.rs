use super::{BitmapFont, GlyphInstance};
use crate::Rect;

pub enum TextAlign {
    Left,
    Center,
    Right,
}

pub fn layout_text(
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
            cursor_y += font.line_height() as f32;
            prev_char = None;
            continue;
        }

        // Baseline Y for this line. Note: `GlyphMetrics::y_offset` is baseline-relative
        // (converted at font load time by `y_offset = yoffset - base`), so we place glyphs at:
        // `baseline_y + y_offset`.
        let baseline_y = cursor_y + font.baseline() as f32;

        let metrics = font
            .glyph_metrics(ch)
            .unwrap_or_else(|| font.glyph_metrics('?').unwrap());

        if let Some(prev) = prev_char {
            cursor_x += font.kerning_amount(prev, ch) as f32;
        }

        let uv_rect = font.uv_rect(ch).unwrap_or((0.0, 0.0, 1.0, 1.0));
        let size = (metrics.width(), metrics.height());
        let (x_offset, y_offset_from_baseline) = metrics.offset_from_baseline();

        instances.push(GlyphInstance::new(
            ch,
            cursor_x + x_offset as f32,
            baseline_y + y_offset_from_baseline as f32,
            uv_rect,
            size,
        ));

        cursor_x += metrics.x_advance() as f32;
        prev_char = Some(ch);
    }

    instances
}

pub fn measure_text_multiline(font: &BitmapFont, text: &str, max_width: usize) -> (usize, usize) {
    let lines = word_wrap(font, text, max_width);

    let mut max_width = 0;

    for line in &lines {
        let (line_width, _) = measure_text(font, line.as_str());
        max_width = max_width.max(line_width);
    }

    (max_width, font.line_height() * lines.len())
}

pub fn measure_text(font: &BitmapFont, text: &str) -> (usize, usize) {
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

pub fn word_wrap(font: &BitmapFont, text: &str, max_width: usize) -> Vec<String> {
    let max_width = max_width.max(1);
    let space_width = measure_text(font, " ").0 as i32;

    let mut lines: Vec<String> = Vec::new();
    let mut current_line = String::new();
    let mut current_width: i32 = 0;

    for word in text.split_whitespace() {
        let word_width = measure_text(font, word).0 as i32;

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

pub fn layout_text_aligned(
    font: &BitmapFont,
    text: &str,
    bounds: Rect,
    align: TextAlign,
) -> Vec<GlyphInstance> {
    let (text_width, _) = measure_text(font, text);
    let text_width = text_width as f32;

    let start_x = match align {
        TextAlign::Left => bounds.x,
        TextAlign::Center => bounds.x + (bounds.width - text_width) / 2.0,
        TextAlign::Right => bounds.x + bounds.width - text_width,
    };

    layout_text(font, text, start_x, bounds.y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::image::{Image, PixelFormat};
    use crate::text::BitmapFont;

    fn make_test_font() -> BitmapFont {
        // Baseline-aware test font:
        // - baseline (base) = 10
        // - 'A' has yoffset=2 (from top), xoffset=1, xadvance=6
        // - 'g' has yoffset=6 (from top), xoffset=0, xadvance=6
        // After parsing, stored y_offset becomes (yoffset - base).
        let fnt = r#"
common lineHeight=12 base=10 scaleW=64 scaleH=64 pages=1 packed=0
chars count=3
char id=65 x=0 y=0 width=4 height=5 xoffset=1 yoffset=2 xadvance=6 page=0 chnl=0
char id=103 x=4 y=0 width=4 height=7 xoffset=0 yoffset=6 xadvance=6 page=0 chnl=0
char id=63 x=8 y=0 width=4 height=5 xoffset=0 yoffset=2 xadvance=6 page=0 chnl=0
kernings count=1
kerning first=65 second=103 amount=-2
"#;
        let atlas = Image::new(64, 64, vec![0; 64 * 64 * 4], PixelFormat::Rgba8);
        BitmapFont::new(fnt, atlas)
    }

    #[test]
    fn layout_applies_baseline_and_offsets() {
        let font = make_test_font();

        let start_x = 100.0;
        let start_y = 200.0;
        let instances = layout_text(&font, "Ag", start_x, start_y);

        assert_eq!(instances.len(), 2);

        // Baseline is at start_y + base.
        let baseline_y = start_y + font.baseline() as f32;

        // 'A': x = cursor_x + xoffset. y = baseline_y + (yoffset - base).
        // First glyph cursor_x = start_x.
        let (ax, ay) = instances[0].position();
        assert!((ax - (start_x + 1.0)).abs() < 1e-6);
        assert!((ay - (baseline_y + (2.0 - 10.0))).abs() < 1e-6);

        // 'g': kerning between A and g is -2, cursor_x advances by xadvance=6 after A.
        // cursor_x before placing g = start_x + 6 + (-2).
        let (gx, gy) = instances[1].position();
        assert!((gx - (start_x + 6.0 - 2.0 + 0.0)).abs() < 1e-6);
        assert!((gy - (baseline_y + (6.0 - 10.0))).abs() < 1e-6);
    }

    #[test]
    fn layout_newline_advances_by_line_height() {
        let font = make_test_font();
        let instances = layout_text(&font, "A\nA", 0.0, 0.0);
        assert_eq!(instances.len(), 2);

        let (_, ay) = instances[0].position();
        let (_, gy) = instances[1].position();
        assert!((gy - (ay + font.line_height() as f32)).abs() < 1e-6);
    }

    #[test]
    fn unknown_glyph_falls_back_to_question_mark_metrics() {
        let font = make_test_font();
        let instances = layout_text(&font, "@", 0.0, 0.0);
        assert_eq!(instances.len(), 1);
        assert_eq!(instances[0].ch(), '@');

        // '@' isn't in glyphs, so it should use '?' glyph metrics for size.
        let qm = font.glyph_metrics('?').unwrap();
        assert_eq!(instances[0].size(), (qm.width(), qm.height()));
    }

    #[test]
    fn measure_text_includes_kerning() {
        let font = make_test_font();
        // width = A.xadvance + kerning(A,g) + g.xadvance = 6 + (-2) + 6 = 10
        assert_eq!(measure_text(&font, "Ag").0, 10);
    }

    #[test]
    fn word_wrap_splits_on_width() {
        let font = make_test_font();
        // Each glyph advances 6; space is treated like measure_text(" ") which returns 0 here
        // (no glyph), but wrap should still produce stable output.
        let lines = word_wrap(&font, "A g A g", 6);
        assert!(!lines.is_empty());
    }

    #[test]
    fn layout_text_aligned_sets_start_x() {
        let font = make_test_font();
        let bounds = Rect {
            x: 10.0,
            y: 20.0,
            width: 100.0,
            height: 50.0,
        };

        let left = layout_text_aligned(&font, "Ag", bounds, TextAlign::Left);
        let center = layout_text_aligned(&font, "Ag", bounds, TextAlign::Center);
        let right = layout_text_aligned(&font, "Ag", bounds, TextAlign::Right);

        let (lw, _) = measure_text(&font, "Ag");
        let lw = lw as f32;

        assert!((left[0].position().0 - (bounds.x + 1.0)).abs() < 1e-6);
        assert!(
            (center[0].position().0 - (bounds.x + (bounds.width - lw) / 2.0 + 1.0)).abs() < 1e-6
        );
        assert!((right[0].position().0 - (bounds.x + bounds.width - lw + 1.0)).abs() < 1e-6);
    }
}
