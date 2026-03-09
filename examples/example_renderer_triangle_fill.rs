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

fn fill_triangle_fill_demos(renderer: &mut Renderer) {
    renderer.clear(Color::DARK_GRAY); // Dark background

    // Demo 1: Basic filled triangles
    // Equilateral triangle
    renderer.fill_triangle(
        Vec2::new(100.0, 100.0), // Top vertex
        Vec2::new(50.0, 200.0),  // Bottom left
        Vec2::new(150.0, 200.0), // Bottom right
        Color::RED,
        Mat3::IDENTITY,
    );

    // Right triangle
    renderer.fill_triangle(
        Vec2::new(250.0, 100.0),
        Vec2::new(250.0, 200.0),
        Vec2::new(350.0, 200.0),
        Color::GREEN,
        Mat3::IDENTITY,
    );

    // Isosceles triangle
    renderer.fill_triangle(
        Vec2::new(450.0, 100.0),
        Vec2::new(400.0, 200.0),
        Vec2::new(500.0, 200.0),
        Color::BLUE,
        Mat3::IDENTITY,
    );

    // Scalene triangle
    renderer.fill_triangle(
        Vec2::new(600.0, 100.0),
        Vec2::new(550.0, 200.0),
        Vec2::new(650.0, 180.0),
        Color::YELLOW,
        Mat3::IDENTITY,
    );

    // Demo 2: Triangle patterns
    // Row of triangles
    for i in 0..8 {
        let x = 50.0 + (i as f32) * 60.0;
        let height = 40.0 + (i as f32) * 5.0;
        let color = match i % 4 {
            0 => Color::RED,
            1 => Color::GREEN,
            2 => Color::BLUE,
            3 => Color::YELLOW,
            _ => Color::WHITE,
        };
        renderer.fill_triangle(
            Vec2::new(x, 250.0),
            Vec2::new(x - 20.0, 250.0 + height),
            Vec2::new(x + 20.0, 250.0 + height),
            color,
            Mat3::IDENTITY,
        );
    }

    // Demo 3: Triangle grid
    for row in 0..4 {
        for col in 0..6 {
            let x = 500.0 + (col as f32) * 60.0;
            let y = 250.0 + (row as f32) * 50.0;
            let color = if (row + col) % 2 == 0 {
                Color::CYAN
            } else {
                Color::MAGENTA
            };
            renderer.fill_triangle(
                Vec2::new(x, y),
                Vec2::new(x - 25.0, y + 40.0),
                Vec2::new(x + 25.0, y + 40.0),
                color,
                Mat3::IDENTITY,
            );
        }
    }

    // Demo 4: Rotating triangles
    let rotation = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as f32)
        * 0.002; // Slower rotation

    // Rotating equilateral triangle
    let center = Vec2::new(200.0, 400.0);
    let radius = 50.0;
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
    renderer.fill_triangle(a, b, c, Color::PINK, Mat3::IDENTITY);

    // Rotating right triangle
    let center2 = Vec2::new(400.0, 400.0);
    let a2 = Vec2::new(
        center2.x + 40.0 * rotation.cos(),
        center2.y + 40.0 * rotation.sin(),
    );
    let b2 = Vec2::new(
        center2.x + 40.0 * (rotation + PI / 2.0).cos(),
        center2.y + 40.0 * (rotation + PI / 2.0).sin(),
    );
    let c2 = center2;
    renderer.fill_triangle(a2, b2, c2, Color::GREEN, Mat3::IDENTITY);

    // Demo 5: Concentric triangles
    let center = Vec2::new(600.0, 400.0);
    for i in 0..6 {
        let size = 60.0 - (i as f32) * 8.0;
        let color = match i % 3 {
            0 => Color::RED,
            1 => Color::GREEN,
            2 => Color::BLUE,
            _ => Color::WHITE,
        };
        renderer.fill_triangle(
            Vec2::new(center.x, center.y - size),
            Vec2::new(center.x - size * 0.866, center.y + size * 0.5),
            Vec2::new(center.x + size * 0.866, center.y + size * 0.5),
            color,
            Mat3::IDENTITY,
        );
    }

    // Demo 6: Triangle gradients (simulated with multiple triangles)
    let base_y = 550.0;
    for i in 0..20 {
        let x = 50.0 + (i as f32) * 15.0;
        let intensity = (i as f32) / 19.0;
        let r = (intensity * 255.0) as u8;
        let g = ((1.0 - intensity) * 255.0) as u8;
        let b = 128;
        let color = Color::RGBA(r, g, b, 255);
        renderer.fill_triangle(
            Vec2::new(x, base_y),
            Vec2::new(x - 7.0, base_y + 30.0),
            Vec2::new(x + 7.0, base_y + 30.0),
            color,
            Mat3::IDENTITY,
        );
    }

    // Demo 7: Triangle outlines vs fills
    // Outline triangle
    renderer.fill_triangle(
        Vec2::new(400.0, 550.0),
        Vec2::new(350.0, 650.0),
        Vec2::new(450.0, 650.0),
        Color::WHITE,
        Mat3::IDENTITY,
    );

    // Filled triangle
    renderer.fill_triangle(
        Vec2::new(500.0, 550.0),
        Vec2::new(450.0, 650.0),
        Vec2::new(550.0, 650.0),
        Color::GREEN,
        Mat3::IDENTITY,
    );

    // Both outline and fill
    renderer.fill_triangle(
        Vec2::new(600.0, 550.0),
        Vec2::new(550.0, 650.0),
        Vec2::new(650.0, 650.0),
        Color::BLUE,
        Mat3::IDENTITY,
    );
    renderer.fill_triangle(
        Vec2::new(600.0, 550.0),
        Vec2::new(550.0, 650.0),
        Vec2::new(650.0, 650.0),
        Color::WHITE,
        Mat3::IDENTITY,
    );

    // Demo 8: Edge cases
    // Very small triangle
    renderer.fill_triangle(
        Vec2::new(750.0, 550.0),
        Vec2::new(745.0, 555.0),
        Vec2::new(755.0, 555.0),
        Color::YELLOW,
        Mat3::IDENTITY,
    );

    // Very thin triangle
    renderer.fill_triangle(
        Vec2::new(800.0, 550.0),
        Vec2::new(800.0, 650.0),
        Vec2::new(810.0, 650.0),
        Color::CYAN,
        Mat3::IDENTITY,
    );

    // Triangle with negative coordinates (should be clipped)
    renderer.fill_triangle(
        Vec2::new(-50.0, 550.0),
        Vec2::new(50.0, 550.0),
        Vec2::new(0.0, 500.0),
        Color::MAGENTA,
        Mat3::IDENTITY,
    );

    // Degenerate triangle (collinear points) - should be skipped
    renderer.fill_triangle(
        Vec2::new(900.0, 550.0),
        Vec2::new(950.0, 550.0),
        Vec2::new(1000.0, 550.0),
        Color::RED,
        Mat3::IDENTITY,
    );
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) = Window::new(
        "lesson_2_2_6 - Triangle Fill Drawing",
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
            fill_triangle_fill_demos(&mut renderer);
        }

        window.present(&frame_buffer)?;

        let frame_time = frame_start.elapsed();
        let target = Duration::from_nanos(1_000_000_000 / 60);
        if let Some(remaining) = target.checked_sub(frame_time) {
            std::thread::sleep(remaining);
        }
    }
}
