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

fn draw_test_lines(renderer: &mut Renderer) {
    renderer.draw_line_aa(
        Vec2::new(10.0, 10.0),
        Vec2::new(2000.0, 1000.0),
        Color::RED,
        Mat3::IDENTITY,
    );
    renderer.draw_line_aa(
        Vec2::new(100.0, 200.0),
        Vec2::new(500.0, 50.0),
        Color::GREEN,
        Mat3::IDENTITY,
    );
    renderer.draw_line_aa(
        Vec2::new(300.0, 300.0),
        Vec2::new(800.0, 600.0),
        Color::BLUE,
        Mat3::IDENTITY,
    );
    renderer.draw_line_aa(
        Vec2::new(1000.0, 100.0),
        Vec2::new(200.0, 500.0),
        Color::YELLOW,
        Mat3::IDENTITY,
    );
    renderer.draw_line_aa(
        Vec2::new(50.0, 600.0),
        Vec2::new(1200.0, 200.0),
        Color::CYAN,
        Mat3::IDENTITY,
    );
    renderer.draw_line_aa(
        Vec2::new(600.0, 50.0),
        Vec2::new(650.0, 650.0),
        Color::MAGENTA,
        Mat3::IDENTITY,
    );
    renderer.draw_line_aa(
        Vec2::new(900.0, 400.0),
        Vec2::new(100.0, 350.0),
        Color::WHITE,
        Mat3::IDENTITY,
    );
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) = Window::new(
        "lesson_2_1_1 - Line Drawing Test (AA)",
        INITIAL_WINDOW_WIDTH as u32,
        INITIAL_WINDOW_HEIGHT as u32,
    )?;

    let mut frame_buffer = FrameBuffer::new(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT);

    // Clear to black background
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
                    // Handle window resize
                    frame_buffer.resize(w as usize, h as usize);
                }
                _ => {}
            }
        }

        {
            let mut renderer = Renderer::new(&mut frame_buffer);
            renderer.clear(Color::BLACK);

            draw_test_lines(&mut renderer);
        }

        // Present the frame
        window.present(&frame_buffer)?;

        // Cap the frame rate
        let frame_time = frame_start.elapsed();
        let target = Duration::from_nanos(1_000_000_000 / 60);
        if let Some(remaining) = target.checked_sub(frame_time) {
            std::thread::sleep(remaining);
        }
    }
}
