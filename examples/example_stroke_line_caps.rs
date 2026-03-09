use scratchpad_rs::Color;
use scratchpad_rs::framebuffer::FrameBuffer;
use scratchpad_rs::math::{Mat3, Vec2};
use scratchpad_rs::renderer::{LineCap, Renderer, StrokeStyle};
use scratchpad_rs::window::Window;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use std::time::Duration;

const INITIAL_WINDOW_WIDTH: usize = 1280;
const INITIAL_WINDOW_HEIGHT: usize = 720;

fn draw_cap_row(renderer: &mut Renderer, y: f32, cap: LineCap, color: Color) {
    let thickness = 22.0;
    let a = Vec2::new(140.0, y);
    let b = Vec2::new(520.0, y);
    let c = Vec2::new(900.0, y);
    let style = StrokeStyle::solid_screen_px(thickness, color).with_cap(cap);

    // Reference centerlines.
    renderer.draw_line_aa(a, b, Color::DARK_GRAY, Mat3::IDENTITY);
    renderer.draw_line_aa(b, c, Color::DARK_GRAY, Mat3::IDENTITY);

    // Horizontal
    renderer.stroke_line(a, b, &style, Mat3::IDENTITY);
    // Diagonal
    renderer.stroke_line(
        Vec2::new(b.x, b.y + 60.0),
        Vec2::new(c.x, c.y - 60.0),
        &style,
        Mat3::IDENTITY,
    );

    // Endpoint markers to visualize cap extension.
    let marker_h = 40.0;
    renderer.draw_line_pixel(
        scratchpad_rs::math::IVec2::new(a.x as i32, (a.y - marker_h) as i32),
        scratchpad_rs::math::IVec2::new(a.x as i32, (a.y + marker_h) as i32),
        Color::WHITE,
        Mat3::IDENTITY,
    );
    renderer.draw_line_pixel(
        scratchpad_rs::math::IVec2::new(b.x as i32, (b.y - marker_h) as i32),
        scratchpad_rs::math::IVec2::new(b.x as i32, (b.y + marker_h) as i32),
        Color::WHITE,
        Mat3::IDENTITY,
    );
}

fn draw_demo(renderer: &mut Renderer) {
    renderer.clear(Color::GRAY);

    draw_cap_row(renderer, 140.0, LineCap::Butt, Color::CYAN);
    draw_cap_row(renderer, 320.0, LineCap::Square, Color::MAGENTA);
    draw_cap_row(renderer, 500.0, LineCap::Round, Color::YELLOW);
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) = Window::new(
        "Lesson 4.3 - Line Caps (Butt / Square / Round)",
        INITIAL_WINDOW_WIDTH as u32,
        INITIAL_WINDOW_HEIGHT as u32,
    )?;

    let mut framebuffer = FrameBuffer::new(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT);
    window.clear(0, 0, 0);

    'running: loop {
        let frame_start = std::time::Instant::now();

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
                Event::Window {
                    win_event: WindowEvent::Resized(width, height),
                    ..
                } => framebuffer.resize(width as usize, height as usize),
                _ => {}
            }
        }

        {
            let mut renderer = Renderer::new(&mut framebuffer);
            draw_demo(&mut renderer);
        }

        window.present(&framebuffer)?;

        let frame_time = frame_start.elapsed();
        let target = Duration::from_nanos(1_000_000_000 / 60);
        if let Some(remaining) = target.checked_sub(frame_time) {
            std::thread::sleep(remaining);
        }
    }
}
