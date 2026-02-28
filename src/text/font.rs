use std::collections::HashMap;

use crate::image::Image;

fn parse_kv(token: &str) -> Option<(&str, &str)> {
    let (key, value) = token.split_once('=')?;
    Some((key, value.trim_matches('"')))
}

pub(crate) struct GlyphMetrics {
    atlas_region_x: usize,
    atlas_region_y: usize,

    atlas_region_width: usize,
    atlas_region_height: usize,

    x_offset: isize, // horizontal offset from the cursor position
    y_offset: isize, // vertical offset from the baseline

    x_advance: usize, // cursor advance after rendering this glyph
}

pub struct BitmapFont {
    texture: Image,
    line_height: usize,
    baseline: usize,
    glyphs: HashMap<char, GlyphMetrics>,
    kerning: HashMap<(char, char), isize>,
}

impl BitmapFont {
    pub fn new(fnt: &str, texture: Image) -> Self {
        let mut glyphs = HashMap::new();
        let mut kerning = HashMap::new();
        let mut line_height = 0usize;
        let mut baseline = 0usize;
        let mut max_glyph_height = 0usize;

        for line in fnt.lines() {
            let mut iter = line.split_whitespace();
            let Some(tag) = iter.next() else {
                continue;
            };

            match tag {
                "info" => {
                    // Parse font info (e.g., size, padding)
                }
                "common" => {
                    for token in iter {
                        let Some((key, value)) = parse_kv(token) else {
                            continue;
                        };
                        match key {
                            "lineHeight" => {
                                if let Ok(v) = value.parse::<usize>() {
                                    line_height = v;
                                }
                            }
                            "base" => {
                                if let Ok(v) = value.parse::<usize>() {
                                    baseline = v;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                "char" => {
                    let mut id: Option<u32> = None;
                    let mut atlas_region_x: Option<usize> = None;
                    let mut atlas_region_y: Option<usize> = None;
                    let mut atlas_region_width: Option<usize> = None;
                    let mut atlas_region_height: Option<usize> = None;
                    let mut x_offset: Option<isize> = None;
                    let mut y_offset: Option<isize> = None;
                    let mut x_advance: Option<usize> = None;

                    for token in iter {
                        let Some((key, value)) = parse_kv(token) else {
                            continue;
                        };
                        match key {
                            "id" => id = value.parse::<u32>().ok(),
                            "x" => atlas_region_x = value.parse::<usize>().ok(),
                            "y" => atlas_region_y = value.parse::<usize>().ok(),
                            "width" => atlas_region_width = value.parse::<usize>().ok(),
                            "height" => atlas_region_height = value.parse::<usize>().ok(),
                            "xoffset" => x_offset = value.parse::<isize>().ok(),
                            "yoffset" => y_offset = value.parse::<isize>().ok(),
                            "xadvance" => x_advance = value.parse::<usize>().ok(),
                            _ => {}
                        }
                    }

                    let (
                        Some(id),
                        Some(atlas_x),
                        Some(atlas_y),
                        Some(atlas_width),
                        Some(atlas_height),
                        Some(x_offset),
                        Some(y_offset),
                        Some(x_advance),
                    ) = (
                        id,
                        atlas_region_x,
                        atlas_region_y,
                        atlas_region_width,
                        atlas_region_height,
                        x_offset,
                        y_offset,
                        x_advance,
                    )
                    else {
                        continue;
                    };

                    let Some(ch) = char::from_u32(id) else {
                        continue;
                    };

                    max_glyph_height = max_glyph_height.max(atlas_height);

                    glyphs.insert(
                        ch,
                        GlyphMetrics {
                            atlas_region_x: atlas_x,
                            atlas_region_y: atlas_y,
                            atlas_region_width: atlas_width,
                            atlas_region_height: atlas_height,
                            x_offset,
                            y_offset,
                            x_advance,
                        },
                    );
                }
                "kerning" => {
                    let mut first: Option<u32> = None;
                    let mut second: Option<u32> = None;
                    let mut amount: Option<isize> = None;

                    for token in iter {
                        let Some((key, value)) = parse_kv(token) else {
                            continue;
                        };
                        match key {
                            "first" => first = value.parse::<u32>().ok(),
                            "second" => second = value.parse::<u32>().ok(),
                            "amount" => amount = value.parse::<isize>().ok(),
                            _ => {}
                        }
                    }

                    let (Some(first), Some(second), Some(amount)) = (first, second, amount) else {
                        continue;
                    };
                    let (Some(first), Some(second)) =
                        (char::from_u32(first), char::from_u32(second))
                    else {
                        continue;
                    };
                    kerning.insert((first, second), amount);
                }
                _ => {}
            }
        }

        if line_height == 0 {
            line_height = max_glyph_height;
        }
        if baseline == 0 {
            baseline = line_height;
        }

        Self {
            texture,
            line_height,
            baseline,
            glyphs,
            kerning,
        }
    }

    #[inline]
    pub fn line_height(&self) -> usize {
        self.line_height
    }

    #[inline]
    pub fn baseline(&self) -> usize {
        self.baseline
    }

    #[inline]
    pub(crate) fn atlas(&self) -> &Image {
        &self.texture
    }

    #[inline]
    pub(crate) fn glyph_metrics(&self, ch: char) -> Option<&GlyphMetrics> {
        self.glyphs.get(&ch)
    }

    #[inline]
    pub(crate) fn kerning_amount(&self, first: char, second: char) -> isize {
        self.kerning.get(&(first, second)).copied().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use crate::image::{Image, PixelFormat};
    use crate::text::BitmapFont;

    #[test]
    fn parses_common_char_and_kerning() {
        let fnt = r#"
info face="Test" size=16
common lineHeight=20 base=15 scaleW=64 scaleH=64 pages=1 packed=0
chars count=2
char id=65 x=1 y=2 width=3 height=4 xoffset=-1 yoffset=2 xadvance=5 page=0 chnl=0
char id=66 x=10 y=20 width=30 height=40 xoffset=0 yoffset=0 xadvance=7 page=0 chnl=0
kernings count=1
kerning first=65 second=66 amount=-2
"#;

        let atlas = Image::new(64, 64, vec![0; 64 * 64 * 4], PixelFormat::Rgba8);
        let font = BitmapFont::new(fnt, atlas);

        assert_eq!(font.line_height, 20);
        assert_eq!(font.baseline, 15);
        assert_eq!(font.glyphs.len(), 2);
        assert_eq!(font.kerning.get(&('A', 'B')).copied(), Some(-2));

        let a = font.glyphs.get(&'A').unwrap();
        assert_eq!(a.atlas_region_x, 1);
        assert_eq!(a.atlas_region_y, 2);
        assert_eq!(a.atlas_region_width, 3);
        assert_eq!(a.atlas_region_height, 4);
        assert_eq!(a.x_offset, -1);
        assert_eq!(a.y_offset, 2);
        assert_eq!(a.x_advance, 5);
    }
}
