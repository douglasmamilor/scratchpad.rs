use scratchpad_rs::Color;
use scratchpad_rs::framebuffer::FrameBuffer;
use scratchpad_rs::math::{Mat3, Vec2};
use scratchpad_rs::renderer::Renderer;
use scratchpad_rs::window::Window;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use std::f32::consts::PI;
use std::time::Duration;

const INITIAL_WINDOW_WIDTH: usize = 1280;
const INITIAL_WINDOW_HEIGHT: usize = 720;

fn draw_triangle_demos(renderer: &mut Renderer) {
    renderer.clear(Color::DARK_GRAY); // Dark background

    // Demo 1: Basic triangle using draw_triangle
    renderer.draw_triangle(
        Vec2::new(100.0, 100.0), // Top vertex
        Vec2::new(50.0, 200.0),  // Bottom left
        Vec2::new(150.0, 200.0), // Bottom right
        Color::RED,
        Mat3::IDENTITY,
    );

    // Demo 2: Different triangle types using draw_triangle
    // Equilateral triangle
    renderer.draw_triangle(
        Vec2::new(300.0, 100.0),
        Vec2::new(350.0, 200.0),
        Vec2::new(400.0, 100.0),
        Color::YELLOW,
        Mat3::IDENTITY,
    );

    // Right triangle
    renderer.draw_triangle(
        Vec2::new(500.0, 100.0),
        Vec2::new(500.0, 200.0),
        Vec2::new(600.0, 200.0),
        Color::CYAN,
        Mat3::IDENTITY,
    );

    // Thin triangle
    renderer.draw_triangle(
        Vec2::new(700.0, 100.0),
        Vec2::new(800.0, 100.0),
        Vec2::new(750.0, 200.0),
        Color::MAGENTA,
        Mat3::IDENTITY,
    );

    // Demo 3: Rotating triangle using draw_triangle
    let rotation = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as f32)
        * 0.001;

    let center = Vec2::new(950.0, 150.0);
    let radius = 40.0;
    let a = Vec2::new(
        center.x + radius * rotation.cos(),
        center.y + radius * rotation.sin(),
    );
    let b = Vec2::new(
        center.x + radius * (rotation + 2.0 * PI / 3.0).cos(),
        center.y + radius * (rotation + 2.0 * PI / 3.0).sin(),
    );
    let c = Vec2::new(
        center.x + radius * (rotation + 4.0 * PI / 3.0).cos(),
        center.y + radius * (rotation + 4.0 * PI / 3.0).sin(),
    );
    renderer.draw_triangle(a, b, c, Color::PINK, Mat3::IDENTITY);

    // Demo 4: Triangle patterns
    // Row of triangles
    for i in 0..8 {
        let x = 100.0 + (i as f32) * 50.0;
        let height = 30.0 + (i as f32) * 5.0;
        renderer.draw_triangle(
            Vec2::new(x, 300.0),
            Vec2::new(x - 20.0, 300.0 + height),
            Vec2::new(x + 20.0, 300.0 + height),
            Color::GREEN,
            Mat3::IDENTITY,
        );
    }

    // Demo 5: Triangle grid
    for row in 0..4 {
        for col in 0..6 {
            let x = 500.0 + (col as f32) * 60.0;
            let y = 300.0 + (row as f32) * 50.0;
            let color = if (row + col) % 2 == 0 {
                Color::BLUE
            } else {
                Color::RED
            };
            renderer.draw_triangle(
                Vec2::new(x, y),
                Vec2::new(x - 25.0, y + 40.0),
                Vec2::new(x + 25.0, y + 40.0),
                color,
                Mat3::IDENTITY,
            );
        }
    }

    // Demo 6: Edge cases
    // Degenerate triangle (collinear points) - should be filtered out by draw_triangle
    renderer.draw_triangle(
        Vec2::new(100.0, 500.0),
        Vec2::new(200.0, 500.0),
        Vec2::new(300.0, 500.0),
        Color::RED,
        Mat3::IDENTITY,
    );

    // Very small triangle
    renderer.draw_triangle(
        Vec2::new(400.0, 500.0),
        Vec2::new(405.0, 505.0),
        Vec2::new(410.0, 500.0),
        Color::YELLOW,
        Mat3::IDENTITY,
    );

    // Triangle with negative coordinates (should be clipped)
    renderer.draw_triangle(
        Vec2::new(-50.0, 500.0),
        Vec2::new(50.0, 500.0),
        Vec2::new(0.0, 450.0),
        Color::CYAN,
        Mat3::IDENTITY,
    );
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) = Window::new(
        "lesson_2_2_4 - Triangle Drawing",
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
            draw_triangle_demos(&mut renderer);
        }

        window.present(&frame_buffer)?;

        let frame_time = frame_start.elapsed();
        let target = Duration::from_nanos(1_000_000_000 / 60);
        if let Some(remaining) = target.checked_sub(frame_time) {
            std::thread::sleep(remaining);
        }
    }
}
