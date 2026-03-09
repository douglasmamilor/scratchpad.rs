use scratchpad_rs::framebuffer::FrameBuffer;
use scratchpad_rs::image::{Image, PixelFormat};
use scratchpad_rs::math::{Mat3, Vec2};
use scratchpad_rs::renderer::Renderer;
use scratchpad_rs::text::{TextAlign, layout_text_block_aligned};
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
        '?' => [0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b00000, 0b00100],
        ' ' => [0, 0, 0, 0, 0, 0, 0],
        _ => pattern_5x7('?'),
    }
}

fn make_embedded_bitmap_font() -> BitmapFont {
    let glyphs: [char; 6] = [' ', 'A', 'B', 'C', 'D', '?'];

    let scale: usize = 4;
    let pattern_w = 5usize;
    let pattern_h = 7usize;
    let glyph_w = pattern_w * scale;
    let glyph_h = pattern_h * scale;

    let padding = 2usize;
    let cell_w = glyph_w + padding * 2;
    let cell_h = glyph_h + padding * 2;

    let cols = 6usize;
    let rows = 1usize;
    let atlas_w = cols * cell_w;
    let atlas_h = rows * cell_h;

    let mut rgba = vec![0u8; atlas_w * atlas_h * 4];

    let line_height = cell_h;
    let base = glyph_h;
    let mut fnt = String::new();
    fnt.push_str(&format!(
        "common lineHeight={} base={} scaleW={} scaleH={} pages=1 packed=0\n",
        line_height, base, atlas_w, atlas_h
    ));
    fnt.push_str(&format!("chars count={}\n", glyphs.len()));

    for (idx, ch) in glyphs.iter().copied().enumerate() {
        let x0 = idx * cell_w + padding;
        let y0 = padding;

        let (w, h) = if ch == ' ' { (0usize, 0usize) } else { (glyph_w, glyph_h) };
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

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) =
        Window::new("Lesson 6.1 - Multiline Alignment (Per Line)", WIDTH as u32, HEIGHT as u32)?;

    let mut framebuffer = FrameBuffer::new(WIDTH, HEIGHT);
    window.clear(0, 0, 0);

    let font = make_embedded_bitmap_font();
    let text = "ABCD\nA\nABC";

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::Window { win_event: WindowEvent::Close, .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running Ok(()),
                Event::Window { win_event: WindowEvent::Resized(w, h), .. } => {
                    framebuffer.resize(w as usize, h as usize);
                }
                _ => {}
            }
        }

        {
            let mut r = Renderer::new(&mut framebuffer);
            r.clear(Color::DARK_GRAY);

            let blocks = [
                (Rect::new(80.0, 80.0, 240.0, 200.0), TextAlign::Left, Color::YELLOW),
                (Rect::new(360.0, 80.0, 240.0, 200.0), TextAlign::Center, Color::CYAN),
                (Rect::new(640.0, 80.0, 240.0, 200.0), TextAlign::Right, Color::MAGENTA),
            ];

            for (bounds, align, tint) in blocks {
                r.draw_rect(
                    Vec2::new(bounds.x, bounds.y),
                    Vec2::new(bounds.x + bounds.width, bounds.y + bounds.height),
                    Color::LIGHT_GRAY,
                    Mat3::IDENTITY,
                );

                let instances = layout_text_block_aligned(&font, text, bounds, align);
                r.render_text_with_shadow(
                    &font,
                    &instances,
                    tint,
                    Vec2::new(2.0, 2.0),
                    Color::RGBA(0, 0, 0, 160),
                    Mat3::IDENTITY,
                );
            }
        }

        window.present(&framebuffer)?;
    }
}
