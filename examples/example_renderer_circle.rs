use scratchpad_rs::Color;
use scratchpad_rs::framebuffer::FrameBuffer;
use scratchpad_rs::math::{Mat3, Vec2};
use scratchpad_rs::renderer::Renderer;
use scratchpad_rs::window::Window;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use std::time::Duration;

const INITIAL_WINDOW_WIDTH: usize = 1280;
const INITIAL_WINDOW_HEIGHT: usize = 720;

fn draw_demo(renderer: &mut Renderer) {
    renderer.clear(Color::BLACK);

    let width = renderer.width() as i32;
    let height = renderer.height() as i32;
    let center = Vec2::new(width as f32 / 2.0, height as f32 / 2.0);

    // Nested fills to show how circle layers stack when rendered back-to-front
    let circle_layers = [
        (120.0, Color::DARK_GRAY),
        (80.0, Color::BLUE),
        (40.0, Color::CYAN),
    ];

    for &(radius, fill_color) in &circle_layers {
        renderer.fill_circle(center, radius, fill_color, Mat3::IDENTITY);
        renderer.draw_circle(center, radius, Color::WHITE, Mat3::IDENTITY);
        renderer.draw_rect(
            Vec2::new(center.x - radius, center.y - radius),
            Vec2::new(center.x + radius, center.y + radius),
            Color::GRAY,
            Mat3::IDENTITY,
        );
    }

    // Standalone filled circles to demonstrate positioning away from the origin
    let filled_samples = [
        (
            Vec2::new(width as f32 / 4.0, height as f32 / 3.0),
            60.0,
            Color::ORANGE,
        ),
        (
            Vec2::new(width as f32 / 2.0, height as f32 / 3.0),
            45.0,
            Color::GREEN,
        ),
        (
            Vec2::new((3.0 * width as f32) / 4.0, height as f32 / 3.0),
            30.0,
            Color::MAGENTA,
        ),
    ];

    for &(ctr, radius, fill_color) in &filled_samples {
        renderer.fill_circle(ctr, radius, fill_color, Mat3::IDENTITY);
        renderer.draw_circle(ctr, radius, Color::WHITE, Mat3::IDENTITY);
    }

    // Demonstrate degenerate case: radius zero -> single pixel
    renderer.draw_circle(Vec2::new(100.0, 100.0), 0.0, Color::WHITE, Mat3::IDENTITY);
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) = Window::new(
        "lesson_2_2_2 - Circle Drawing Demo",
        INITIAL_WINDOW_WIDTH as u32,
        INITIAL_WINDOW_HEIGHT as u32,
    )?;

    let mut frame_buffer = FrameBuffer::new(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT);

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
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => {
                    frame_buffer.resize(w as usize, h as usize);
                }
                _ => {}
            }
        }

        {
            let mut renderer = Renderer::new(&mut frame_buffer);
            draw_demo(&mut renderer);
        }

        window.present(&frame_buffer)?;

        let frame_time = frame_start.elapsed();
        let target = Duration::from_nanos(1_000_000_000 / 60);
        if let Some(remaining) = target.checked_sub(frame_time) {
            std::thread::sleep(remaining);
        }
    }
}
