use scratchpad_rs::Color;
use scratchpad_rs::framebuffer::FrameBuffer;
use scratchpad_rs::math::{IVec2, Mat3};
use scratchpad_rs::renderer::Renderer;
use scratchpad_rs::window::Window;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use std::time::Duration;

const INITIAL_WINDOW_WIDTH: usize = 1280;
const INITIAL_WINDOW_HEIGHT: usize = 720;

fn draw_test_lines(renderer: &mut Renderer) {
    renderer.draw_line_pixel(
        IVec2::new(10, 10),
        IVec2::new(2000, 1000),
        Color::RED,
        Mat3::IDENTITY,
    );
    renderer.draw_line_pixel(
        IVec2::new(100, 200),
        IVec2::new(500, 50),
        Color::GREEN,
        Mat3::IDENTITY,
    );
    renderer.draw_line_pixel(
        IVec2::new(300, 300),
        IVec2::new(800, 600),
        Color::BLUE,
        Mat3::IDENTITY,
    );
    renderer.draw_line_pixel(
        IVec2::new(1000, 100),
        IVec2::new(200, 500),
        Color::YELLOW,
        Mat3::IDENTITY,
    );
    renderer.draw_line_pixel(
        IVec2::new(50, 600),
        IVec2::new(1200, 200),
        Color::CYAN,
        Mat3::IDENTITY,
    );
    renderer.draw_line_pixel(
        IVec2::new(600, 50),
        IVec2::new(650, 650),
        Color::MAGENTA,
        Mat3::IDENTITY,
    );
    renderer.draw_line_pixel(
        IVec2::new(900, 400),
        IVec2::new(100, 350),
        Color::WHITE,
        Mat3::IDENTITY,
    );
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) = Window::new(
        "lesson_2_1 - Line Drawing Test",
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
