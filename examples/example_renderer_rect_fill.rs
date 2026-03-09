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

fn fill_rectangles(renderer: &mut Renderer) {
    renderer.clear(Color::BLACK);

    let colors = [
        Color::RED,
        Color::GREEN,
        Color::BLUE,
        Color::YELLOW,
        Color::CYAN,
        Color::MAGENTA,
    ];

    let max_inset = ((renderer.width().min(renderer.height()) as i32) / 2).max(1);
    let step = 40;

    for (idx, color) in colors.iter().enumerate() {
        let inset = (idx as i32) * step;
        if inset >= max_inset {
            break;
        }

        let available_width = renderer.width() as i32 - inset * 2;
        let available_height = renderer.height() as i32 - inset * 2;

        if available_width <= 0 || available_height <= 0 {
            break;
        }

        renderer.fill_rect(
            Vec2::new(inset as f32, inset as f32),
            Vec2::new(
                (inset + available_width) as f32,
                (inset + available_height) as f32,
            ),
            *color,
            Mat3::IDENTITY,
        );
    }

    // Draw a grid of small rectangles in the center to demonstrate degenerate cases
    let cell_size: usize = 32;
    let cell_stride = cell_size as i32 + 10;
    let origin_x = renderer.width() as i32 / 2 - cell_size as i32;
    let origin_y = renderer.height() as i32 / 2 - cell_size as i32;

    for row in 0..3 {
        for col in 0..3 {
            let x = origin_x + col as i32 * cell_stride;
            let y = origin_y + row as i32 * cell_stride;
            renderer.fill_rect(
                Vec2::new(x as f32, y as f32),
                Vec2::new((x + cell_size as i32) as f32, (y + cell_size as i32) as f32),
                Color::LIGHT_GRAY,
                Mat3::IDENTITY,
            );
        }
    }

    // Single pixel rectangle to show degenerate handling
    renderer.fill_rect(
        Vec2::new(
            renderer.width() as f32 / 2.0,
            renderer.height() as f32 / 2.0,
        ),
        Vec2::new(
            renderer.width() as f32 / 2.0 + 1.0,
            renderer.height() as f32 / 2.0 + 1.0,
        ),
        Color::WHITE,
        Mat3::IDENTITY,
    );
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) = Window::new(
        "lesson_2_2 - Rectangle Drawing",
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
            fill_rectangles(&mut renderer);
        }

        window.present(&frame_buffer)?;

        let frame_time = frame_start.elapsed();
        let target = Duration::from_nanos(1_000_000_000 / 60);
        if let Some(remaining) = target.checked_sub(frame_time) {
            std::thread::sleep(remaining);
        }
    }
}
