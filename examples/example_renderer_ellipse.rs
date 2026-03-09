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

fn draw_ellipses(renderer: &mut Renderer) {
    renderer.clear(Color::BLACK);

    let center = Vec2::new(
        renderer.width() as f32 / 2.0,
        renderer.height() as f32 / 2.0,
    );

    // Draw a variety of ellipses to demonstrate the algorithm
    let colors = [
        Color::RED,
        Color::GREEN,
        Color::BLUE,
        Color::YELLOW,
        Color::CYAN,
        Color::MAGENTA,
        Color::ORANGE,
        Color::PURPLE,
    ];

    // 1. Concentric ellipses with different aspect ratios
    for (i, color) in colors.iter().enumerate() {
        let radius_x = 50 + (i as i32) * 30;
        let radius_y = 30 + (i as i32) * 20;

        renderer.draw_ellipse(
            center,
            radius_x as f32,
            radius_y as f32,
            *color,
            Mat3::IDENTITY,
        );
    }

    // 2. Horizontal ellipses (wide)
    for i in 0..5 {
        let x = 100 + i as i32 * 200;
        let y = 100;
        let radius_x = 80;
        let radius_y = 40;

        renderer.draw_ellipse(
            Vec2::new(x as f32, y as f32),
            radius_x as f32,
            radius_y as f32,
            Color::LIGHT_GRAY,
            Mat3::IDENTITY,
        );
    }

    // 3. Vertical ellipses (tall)
    for i in 0..5 {
        let x = 100 + i as i32 * 200;
        let y = 600;
        let radius_x = 40;
        let radius_y = 80;

        renderer.draw_ellipse(
            Vec2::new(x as f32, y as f32),
            radius_x as f32,
            radius_y as f32,
            Color::DARK_GRAY,
            Mat3::IDENTITY,
        );
    }

    // 4. Degenerate cases
    // Single point
    renderer.draw_ellipse(
        Vec2::new(50.0, 50.0),
        0.0,
        0.0,
        Color::WHITE,
        Mat3::IDENTITY,
    );

    // Horizontal line
    renderer.draw_ellipse(
        Vec2::new(150.0, 50.0),
        20.0,
        0.0,
        Color::WHITE,
        Mat3::IDENTITY,
    );

    // Vertical line
    renderer.draw_ellipse(
        Vec2::new(250.0, 50.0),
        0.0,
        20.0,
        Color::WHITE,
        Mat3::IDENTITY,
    );

    // 5. Circles (special case of ellipses with equal radii)
    for i in 0..3 {
        let x = 1000 + i as i32 * 80;
        let y = 200;
        let radius = 30 + i as i32 * 10;

        renderer.draw_ellipse(
            Vec2::new(x as f32, y as f32),
            radius as f32,
            radius as f32,
            Color::PINK,
            Mat3::IDENTITY,
        );
    }

    // 6. Very thin ellipses to test edge cases
    renderer.draw_ellipse(
        Vec2::new(1000.0, 400.0),
        60.0,
        5.0,
        Color::BROWN,
        Mat3::IDENTITY,
    );
    renderer.draw_ellipse(
        Vec2::new(1000.0, 450.0),
        5.0,
        60.0,
        Color::BROWN,
        Mat3::IDENTITY,
    );

    // 7. Large ellipses to test performance
    renderer.draw_ellipse(center, 200.0, 150.0, Color::GRAY, Mat3::IDENTITY);
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) = Window::new(
        "lesson_2_3 - Ellipse Drawing",
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
            draw_ellipses(&mut renderer);
        }

        window.present(&frame_buffer)?;

        let frame_time = frame_start.elapsed();
        let target = Duration::from_nanos(1_000_000_000 / 60);
        if let Some(remaining) = target.checked_sub(frame_time) {
            std::thread::sleep(remaining);
        }
    }
}
