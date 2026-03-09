use scratchpad_rs::asset::AssetLoader;
use scratchpad_rs::framebuffer::FrameBuffer;
use scratchpad_rs::image::{Image, PixelFormat};
use scratchpad_rs::math::{Mat3, Vec2};
use scratchpad_rs::renderer::Renderer;
use scratchpad_rs::text::{
    TextAlign, layout_text_anchored, layout_text_wrapped_aligned, measure_text_block,
};
use scratchpad_rs::ui::Anchor;
use scratchpad_rs::window::Window;
use scratchpad_rs::{BitmapFont, Color, Rect};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;

const WIDTH: usize = 960;
const HEIGHT: usize = 540;

fn pattern_5x7(ch: char) -> [u8; 7] {
    match ch {
        'A' => [0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001],
        'B' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110],
        'C' => [0b01110, 0b10001, 0b10000, 0b10000, 0b10000, 0b10001, 0b01110],
        'D' => [0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110],
        'E' => [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111],
        'F' => [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000],
        'G' => [0b01110, 0b10001, 0b10000, 0b10111, 0b10001, 0b10001, 0b01110],
        'H' => [0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001],
        'I' => [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b11111],
        'J' => [0b00111, 0b00010, 0b00010, 0b00010, 0b10010, 0b10010, 0b01100],
        'K' => [0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001],
        'L' => [0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111],
        'M' => [0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001],
        'N' => [0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001],
        'O' => [0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
        'P' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000],
        'Q' => [0b01110, 0b10001, 0b10001, 0b10001, 0b10101, 0b10010, 0b01101],
        'R' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001],
        'S' => [0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110],
        'T' => [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100],
        'U' => [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
        'V' => [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100],
        'W' => [0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b10101, 0b01010],
        'X' => [0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001],
        'Y' => [0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100],
        'Z' => [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111],

        '0' => [0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110],
        '1' => [0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110],
        '2' => [0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b01000, 0b11111],
        '3' => [0b01110, 0b10001, 0b00001, 0b00110, 0b00001, 0b10001, 0b01110],
        '4' => [0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010],
        '5' => [0b11111, 0b10000, 0b11110, 0b00001, 0b00001, 0b10001, 0b01110],
        '6' => [0b00110, 0b01000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110],
        '7' => [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000],
        '8' => [0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110],
        '9' => [0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00010, 0b01100],

        '.' => [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00100, 0b00100],
        '-' => [0b00000, 0b00000, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000],
        '!' => [0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00000, 0b00100],
        '?' => [0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b00000, 0b00100],
        ':' => [0b00000, 0b00100, 0b00100, 0b00000, 0b00100, 0b00100, 0b00000],
        ' ' => [0, 0, 0, 0, 0, 0, 0],
        _ => pattern_5x7('?'),
    }
}

fn make_embedded_bitmap_font() -> BitmapFont {
    let glyphs: Vec<char> = {
        let mut out = Vec::new();
        out.push(' ');
        for ch in 'A'..='Z' {
            out.push(ch);
        }
        for ch in '0'..='9' {
            out.push(ch);
        }
        out.extend(['.', '-', '!', '?', ':']);
        out
    };

    let scale: usize = 2;
    let pattern_w = 5usize;
    let pattern_h = 7usize;
    let glyph_w = pattern_w * scale;
    let glyph_h = pattern_h * scale;

    let padding = 1usize;
    let cell_w = glyph_w + padding * 2;
    let cell_h = glyph_h + padding * 2;

    let cols = 16usize;
    let rows = glyphs.len().div_ceil(cols);
    let atlas_w = cols * cell_w;
    let atlas_h = rows * cell_h;

    let mut rgba = vec![0u8; atlas_w * atlas_h * 4];

    let mut fnt = String::new();
    let line_height = cell_h;
    let base = glyph_h;

    fnt.push_str(&format!(
        "common lineHeight={} base={} scaleW={} scaleH={} pages=1 packed=0\n",
        line_height, base, atlas_w, atlas_h
    ));
    fnt.push_str(&format!("chars count={}\n", glyphs.len()));

    for (idx, ch) in glyphs.iter().copied().enumerate() {
        let col = idx % cols;
        let row = idx / cols;
        let x0 = col * cell_w + padding;
        let y0 = row * cell_h + padding;

        let (w, h) = if ch == ' ' { (0usize, 0usize) } else { (glyph_w, glyph_h) };

        // Monospace: xadvance equals cell width, so spacing matches the grid.
        let xadvance = cell_w as i32;

        fnt.push_str(&format!(
            "char id={} x={} y={} width={} height={} xoffset=0 yoffset=0 xadvance={} page=0 chnl=0\n",
            ch as u32, x0, y0, w, h, xadvance
        ));

        if ch == ' ' {
            continue;
        }

        let rows_bits = pattern_5x7(ch);
        for (py, bits) in rows_bits.iter().copied().enumerate() {
            for px in 0..pattern_w {
                let on = (bits >> (pattern_w - 1 - px)) & 1 == 1;
                if !on {
                    continue;
                }

                // Scale each bitmap pixel into a scale×scale block in the atlas.
                for sy in 0..scale {
                    for sx in 0..scale {
                        let ax = x0 + px * scale + sx;
                        let ay = y0 + py * scale + sy;
                        let i = (ax + ay * atlas_w) * 4;
                        rgba[i] = 255;
                        rgba[i + 1] = 255;
                        rgba[i + 2] = 255;
                        rgba[i + 3] = 255;
                    }
                }
            }
        }
    }

    let atlas = Image::new(atlas_w, atlas_h, rgba, PixelFormat::Rgba8);
    BitmapFont::new(fnt.as_str(), atlas)
}

fn load_font_or_fallback() -> BitmapFont {
    // If you generate a BMFont atlas + `.fnt` locally, you can point these paths at it.
    // Example expected layout (not included in repo by default):
    // - assets/fonts/your_font.fnt
    // - assets/fonts/your_font.bmp
    AssetLoader::load_bmp_font("assets/fonts/font.fnt", "assets/fonts/font.bmp")
        .unwrap_or_else(|_| make_embedded_bitmap_font())
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) =
        Window::new("Lesson 6.1 - Bitmap Text (Layout + Tint + Shadow)", WIDTH as u32, HEIGHT as u32)?;

    let mut framebuffer = FrameBuffer::new(WIDTH, HEIGHT);
    window.clear(0, 0, 0);

    let font = load_font_or_fallback();

    let mut align = TextAlign::Left;
    let mut shadow = true;
    let mut tint = Color::WHITE;

    println!("Controls:");
    println!("  1/2/3: Left/Center/Right");
    println!("  S: toggle shadow");
    println!("  T: cycle tint");
    println!("  ESC: quit");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::Window { win_event: WindowEvent::Close, .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running Ok(()),
                Event::Window { win_event: WindowEvent::Resized(w, h), .. } => {
                    framebuffer.resize(w as usize, h as usize);
                }
                Event::KeyDown { keycode: Some(Keycode::Num1), .. } => align = TextAlign::Left,
                Event::KeyDown { keycode: Some(Keycode::Num2), .. } => align = TextAlign::Center,
                Event::KeyDown { keycode: Some(Keycode::Num3), .. } => align = TextAlign::Right,
                Event::KeyDown { keycode: Some(Keycode::S), .. } => shadow = !shadow,
                Event::KeyDown { keycode: Some(Keycode::T), .. } => {
                    tint = match tint {
                        Color::WHITE => Color::YELLOW,
                        Color::YELLOW => Color::CYAN,
                        Color::CYAN => Color::MAGENTA,
                        _ => Color::WHITE,
                    };
                }
                _ => {}
            }
        }

        {
            let mut r = Renderer::new(&mut framebuffer);
            r.clear(Color::DARK_GRAY);

            let bounds = Rect::new(80.0, 80.0, 800.0, 220.0);
            r.draw_rect(
                Vec2::new(bounds.x, bounds.y),
                Vec2::new(bounds.x + bounds.width, bounds.y + bounds.height),
                Color::LIGHT_GRAY,
                Mat3::IDENTITY,
            );

            let body = "WRAP  PRESERVES   SPACES\nAND NEWLINES.  1 2 3";
            let instances = layout_text_wrapped_aligned(&font, body, bounds, align);

            if shadow {
                r.render_text_with_shadow(
                    &font,
                    &instances,
                    tint,
                    Vec2::new(2.0, 2.0),
                    Color::RGBA(0, 0, 0, 160),
                    Mat3::IDENTITY,
                );
            } else {
                r.render_text_tinted(&font, &instances, tint, Mat3::IDENTITY);
            }

            // Anchor demo: center a short label.
            let label = "ANCHOR CENTER";
            let anchored = layout_text_anchored(
                &font,
                label,
                Anchor::Center,
                Vec2::new(r.width() as f32 * 0.5, r.height() as f32 * 0.85),
            );

            r.render_text_with_shadow(
                &font,
                &anchored,
                Color::WHITE,
                Vec2::new(2.0, 2.0),
                Color::RGBA(0, 0, 0, 180),
                Mat3::IDENTITY,
            );

            // Draw a debug point at the anchor location.
            let (w, h) = measure_text_block(&font, label);
            let top_left = Anchor::Center.top_left_for(
                Vec2::new(r.width() as f32 * 0.5, r.height() as f32 * 0.85),
                w as f32,
                h as f32,
            );
            let center = top_left + Vec2::new(w as f32 * 0.5, h as f32 * 0.5);
            r.fill_rect(
                center - Vec2::new(2.0, 2.0),
                center + Vec2::new(2.0, 2.0),
                Color::ORANGE,
                Mat3::IDENTITY,
            );
        }

        window.present(&framebuffer)?;
    }
}

