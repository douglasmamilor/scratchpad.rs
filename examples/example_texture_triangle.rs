use scratchpad_rs::Color;
use scratchpad_rs::framebuffer::FrameBuffer;
use scratchpad_rs::image::{Image, PixelFormat};
use scratchpad_rs::math::{Mat3, Vec2};
use scratchpad_rs::renderer::{Renderer, SamplingMode, Texture};
use scratchpad_rs::window::Window;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;

const WIDTH: usize = 960;
const HEIGHT: usize = 720;

fn make_checker_texture() -> Texture {
    // 4x4 RGBA checker in a CMYK-ish palette.
    let w = 4;
    let h = 4;
    let mut data = Vec::with_capacity(w * h * 4);

    let cyan = [0u8, 255, 255, 255];
    let magenta = [255u8, 0, 255, 255];
    let yellow = [255u8, 255, 0, 255];
    let black = [0u8, 0, 0, 255];

    let rows = [
        [cyan, magenta, cyan, magenta],
        [yellow, black, yellow, black],
        [cyan, magenta, cyan, magenta],
        [yellow, black, yellow, black],
    ];

    for row in rows.iter() {
        for px in row {
            data.extend_from_slice(px);
        }
    }

    let img = Image::new(w, h, data, PixelFormat::Rgba8);
    img.into()
}

fn draw_textured_triangle(renderer: &mut Renderer, tex: &Texture, sampling: SamplingMode) {
    renderer.clear(Color::DARK_GRAY);

    // Big triangle covering most of the viewport.
    let a = Vec2::new(100.0, 100.0);
    let b = Vec2::new(840.0, 160.0);
    let c = Vec2::new(200.0, 640.0);

    let uv_a = Vec2::new(0.0, 0.0);
    let uv_b = Vec2::new(1.0, 0.0);
    let uv_c = Vec2::new(0.0, 1.0);

    renderer.fill_triangle_textured(a, b, c, uv_a, uv_b, uv_c, tex, sampling, Mat3::IDENTITY);
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) = Window::new(
        "Lesson 5.2 - Textured Triangles (Nearest vs Bilinear)",
        WIDTH as u32,
        HEIGHT as u32,
    )?;

    let mut framebuffer = FrameBuffer::new(WIDTH, HEIGHT);
    window.clear(0, 0, 0);

    let texture = make_checker_texture();
    let mut sampling = SamplingMode::Nearest;

    println!("Press 'F' to toggle sampling (Nearest/Bilinear), ESC to quit.");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::Window {
                    win_event: WindowEvent::Close,
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running Ok(()),
                Event::KeyDown {
                    keycode: Some(Keycode::F),
                    ..
                } => {
                    sampling = match sampling {
                        SamplingMode::Nearest => SamplingMode::Bilinear,
                        SamplingMode::Bilinear => SamplingMode::Nearest,
                    };
                    println!("Sampling: {:?}", sampling);
                }
                Event::Window {
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => framebuffer.resize(w as usize, h as usize),
                _ => {}
            }
        }

        {
            let mut renderer = Renderer::new(&mut framebuffer);
            draw_textured_triangle(&mut renderer, &texture, sampling);
        }

        window.present(&framebuffer)?;
    }
}
