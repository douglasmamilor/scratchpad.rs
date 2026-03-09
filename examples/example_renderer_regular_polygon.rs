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

fn draw_regular_polygon_demos(renderer: &mut Renderer) {
    renderer.clear(Color::DARK_GRAY); // Dark background

    // Demo 1: Basic regular polygons
    // Triangle (3 sides)
    renderer.draw_regular_polygon(
        Vec2::new(100.0, 100.0),
        50.0,
        0.0,
        3,
        Color::RED,
        Mat3::IDENTITY,
    );

    // Square (4 sides)
    renderer.draw_regular_polygon(
        Vec2::new(250.0, 100.0),
        50.0,
        0.0,
        4,
        Color::GREEN,
        Mat3::IDENTITY,
    );

    // Pentagon (5 sides)
    renderer.draw_regular_polygon(
        Vec2::new(400.0, 100.0),
        50.0,
        0.0,
        5,
        Color::BLUE,
        Mat3::IDENTITY,
    );

    // Hexagon (6 sides)
    renderer.draw_regular_polygon(
        Vec2::new(550.0, 100.0),
        50.0,
        0.0,
        6,
        Color::YELLOW,
        Mat3::IDENTITY,
    );

    // Octagon (8 sides)
    renderer.draw_regular_polygon(
        Vec2::new(700.0, 100.0),
        50.0,
        0.0,
        8,
        Color::CYAN,
        Mat3::IDENTITY,
    );

    // Dodecagon (12 sides) - looks like a circle
    renderer.draw_regular_polygon(
        Vec2::new(850.0, 100.0),
        50.0,
        0.0,
        12,
        Color::MAGENTA,
        Mat3::IDENTITY,
    );

    // Demo 2: Different sizes
    for i in 0..6 {
        let radius = 20.0 + (i as f32) * 15.0;
        let x = 100.0 + (i as f32) * 100.0;
        let color = match i {
            0 => Color::RED,
            1 => Color::GREEN,
            2 => Color::BLUE,
            3 => Color::YELLOW,
            4 => Color::CYAN,
            5 => Color::MAGENTA,
            _ => Color::WHITE,
        };
        renderer.draw_regular_polygon(Vec2::new(x, 250.0), radius, 0.0, 6, color, Mat3::IDENTITY);
    }

    // Demo 3: Different rotations
    for i in 0..8 {
        let angle = (i as f32) * PI / 4.0; // 45 degree increments
        let x = 100.0 + (i as f32) * 100.0;
        renderer.draw_regular_polygon(
            Vec2::new(x, 400.0),
            40.0,
            angle,
            4,
            Color::ORANGE,
            Mat3::IDENTITY,
        );
    }

    // Demo 4: Rotating polygons
    let rotation = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as f32)
        * 0.002; // Slower rotation

    // Rotating triangle
    renderer.draw_regular_polygon(
        Vec2::new(200.0, 550.0),
        60.0,
        rotation,
        3,
        Color::PINK,
        Mat3::IDENTITY,
    );

    // Rotating pentagon
    renderer.draw_regular_polygon(
        Vec2::new(400.0, 550.0),
        60.0,
        rotation,
        5,
        Color::GREEN,
        Mat3::IDENTITY,
    );

    // Rotating octagon
    renderer.draw_regular_polygon(
        Vec2::new(600.0, 550.0),
        60.0,
        rotation,
        8,
        Color::YELLOW,
        Mat3::IDENTITY,
    );

    // Demo 5: Concentric polygons
    let center = Vec2::new(800.0, 550.0);
    for i in 0..5 {
        let radius = 30.0 + (i as f32) * 20.0;
        let sides = 3 + i * 2; // 3, 5, 7, 9, 11 sides
        let color = match i {
            0 => Color::RED,
            1 => Color::GREEN,
            2 => Color::BLUE,
            3 => Color::YELLOW,
            4 => Color::CYAN,
            _ => Color::WHITE,
        };
        renderer.draw_regular_polygon(center, radius, 0.0, sides, color, Mat3::IDENTITY);
    }

    // Demo 6: Polygon patterns
    // Grid of hexagons
    for row in 0..3 {
        for col in 0..4 {
            let x = 100.0 + (col as f32) * 80.0;
            let y = 650.0 + (row as f32) * 60.0;
            let color = if (row + col) % 2 == 0 {
                Color::BLUE
            } else {
                Color::RED
            };
            renderer.draw_regular_polygon(Vec2::new(x, y), 30.0, 0.0, 6, color, Mat3::IDENTITY);
        }
    }

    // Demo 7: High-resolution "circles" (many-sided polygons)
    // 32-sided polygon (looks like a circle)
    renderer.draw_regular_polygon(
        Vec2::new(500.0, 650.0),
        50.0,
        0.0,
        32,
        Color::YELLOW,
        Mat3::IDENTITY,
    );

    // 64-sided polygon (very smooth circle)
    renderer.draw_regular_polygon(
        Vec2::new(600.0, 650.0),
        50.0,
        0.0,
        64,
        Color::CYAN,
        Mat3::IDENTITY,
    );

    // 128-sided polygon (nearly perfect circle)
    renderer.draw_regular_polygon(
        Vec2::new(700.0, 650.0),
        50.0,
        0.0,
        128,
        Color::MAGENTA,
        Mat3::IDENTITY,
    );

    // Demo 8: Edge cases
    // Very small polygon
    renderer.draw_regular_polygon(
        Vec2::new(100.0, 750.0),
        5.0,
        0.0,
        6,
        Color::WHITE,
        Mat3::IDENTITY,
    );

    // Very large polygon
    renderer.draw_regular_polygon(
        Vec2::new(300.0, 750.0),
        100.0,
        0.0,
        6,
        Color::WHITE,
        Mat3::IDENTITY,
    );

    // Minimum sides (triangle)
    renderer.draw_regular_polygon(
        Vec2::new(500.0, 750.0),
        40.0,
        0.0,
        3,
        Color::WHITE,
        Mat3::IDENTITY,
    );

    // Many sides (circle-like)
    renderer.draw_regular_polygon(
        Vec2::new(700.0, 750.0),
        40.0,
        0.0,
        50,
        Color::WHITE,
        Mat3::IDENTITY,
    );
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) = Window::new(
        "lesson_2_2_5 - Regular Polygon Drawing",
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
            draw_regular_polygon_demos(&mut renderer);
        }

        window.present(&frame_buffer)?;

        let frame_time = frame_start.elapsed();
        let target = Duration::from_nanos(1_000_000_000 / 60);
        if let Some(remaining) = target.checked_sub(frame_time) {
            std::thread::sleep(remaining);
        }
    }
}
