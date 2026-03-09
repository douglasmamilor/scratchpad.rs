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

fn draw_barycentric_demos(renderer: &mut Renderer) {
    renderer.clear(Color::DARK_GRAY);

    // Demo 1: Basic color gradients
    // Red -> Green -> Blue triangle
    renderer.fill_triangle_colored(
        Vec2::new(100.0, 100.0), // Top vertex
        Vec2::new(50.0, 200.0),  // Bottom left
        Vec2::new(150.0, 200.0), // Bottom right
        Color::RED,
        Color::GREEN,
        Color::BLUE,
        Mat3::IDENTITY,
    );

    // Yellow -> Cyan -> Magenta triangle
    renderer.fill_triangle_colored(
        Vec2::new(250.0, 100.0),
        Vec2::new(200.0, 200.0),
        Vec2::new(300.0, 200.0),
        Color::YELLOW,
        Color::CYAN,
        Color::MAGENTA,
        Mat3::IDENTITY,
    );

    // Demo 2: Smooth color transitions
    // White -> Red -> Blue
    renderer.fill_triangle_colored(
        Vec2::new(400.0, 100.0),
        Vec2::new(350.0, 200.0),
        Vec2::new(450.0, 200.0),
        Color::WHITE,
        Color::RED,
        Color::BLUE,
        Mat3::IDENTITY,
    );

    // Demo 3: Single color (should match regular fill_triangle)
    renderer.fill_triangle_colored(
        Vec2::new(550.0, 100.0),
        Vec2::new(500.0, 200.0),
        Vec2::new(600.0, 200.0),
        Color::GREEN,
        Color::GREEN,
        Color::GREEN,
        Mat3::IDENTITY,
    );

    // Demo 4: Gradient from one color to another (two vertices same)
    renderer.fill_triangle_colored(
        Vec2::new(700.0, 100.0),
        Vec2::new(650.0, 200.0),
        Vec2::new(750.0, 200.0),
        Color::RED,
        Color::BLUE,
        Color::BLUE,
        Mat3::IDENTITY,
    );

    // Demo 5: Rotating triangle with color gradient
    let rotation = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as f32)
        * 0.001;

    let center = Vec2::new(200.0, 400.0);
    let radius = 60.0;
    let a = Vec2::new(
        center.x + radius * rotation.cos(),
        center.y + radius * rotation.sin(),
    );
    let b = Vec2::new(
        center.x + radius * (rotation + 2.0 * std::f32::consts::PI / 3.0).cos(),
        center.y + radius * (rotation + 2.0 * std::f32::consts::PI / 3.0).sin(),
    );
    let c = Vec2::new(
        center.x + radius * (rotation + 4.0 * std::f32::consts::PI / 3.0).cos(),
        center.y + radius * (rotation + 4.0 * std::f32::consts::PI / 3.0).sin(),
    );

    renderer.fill_triangle_colored(
        a,
        b,
        c,
        Color::RED,
        Color::GREEN,
        Color::BLUE,
        Mat3::IDENTITY,
    );

    // Demo 6: Grid of triangles with different color combinations
    for row in 0..3 {
        for col in 0..4 {
            let x = 400.0 + (col as f32) * 100.0;
            let y = 300.0 + (row as f32) * 80.0;
            let size = 40.0;

            let color1 = match (row + col) % 3 {
                0 => Color::RED,
                1 => Color::GREEN,
                2 => Color::BLUE,
                _ => Color::WHITE,
            };
            let color2 = match (row + col + 1) % 3 {
                0 => Color::RED,
                1 => Color::GREEN,
                2 => Color::BLUE,
                _ => Color::WHITE,
            };
            let color3 = match (row + col + 2) % 3 {
                0 => Color::RED,
                1 => Color::GREEN,
                2 => Color::BLUE,
                _ => Color::WHITE,
            };

            renderer.fill_triangle_colored(
                Vec2::new(x, y),
                Vec2::new(x - size * 0.866, y + size * 0.5),
                Vec2::new(x + size * 0.866, y + size * 0.5),
                color1,
                color2,
                color3,
                Mat3::IDENTITY,
            );
        }
    }

    // Demo 7: Smooth gradient across large triangle
    renderer.fill_triangle_colored(
        Vec2::new(640.0, 50.0),   // Top center
        Vec2::new(100.0, 600.0),  // Bottom left
        Vec2::new(1180.0, 600.0), // Bottom right
        Color::RED,
        Color::GREEN,
        Color::BLUE,
        Mat3::IDENTITY,
    );

    // Demo 8: Subtle color variations (pastel colors)
    let pastel_red = Color::RGBA(255, 182, 193, 255); // Pink
    let pastel_green = Color::RGBA(144, 238, 144, 255); // Light green
    let pastel_blue = Color::RGBA(173, 216, 230, 255); // Light blue

    renderer.fill_triangle_colored(
        Vec2::new(100.0, 650.0),
        Vec2::new(50.0, 700.0),
        Vec2::new(150.0, 700.0),
        pastel_red,
        pastel_green,
        pastel_blue,
        Mat3::IDENTITY,
    );

    // Demo 9: Alpha blending demonstration
    let semi_transparent_red = Color::RGBA(255, 0, 0, 128);
    let semi_transparent_green = Color::RGBA(0, 255, 0, 128);
    let semi_transparent_blue = Color::RGBA(0, 0, 255, 128);

    renderer.fill_triangle_colored(
        Vec2::new(250.0, 650.0),
        Vec2::new(200.0, 700.0),
        Vec2::new(300.0, 700.0),
        semi_transparent_red,
        semi_transparent_green,
        semi_transparent_blue,
        Mat3::IDENTITY,
    );

    // Demo 10: Color wheel effect (multiple triangles)
    let wheel_center = Vec2::new(950.0, 400.0);
    let wheel_radius = 80.0;
    let num_segments = 12;

    for i in 0..num_segments {
        let angle1 = (i as f32) * 2.0 * std::f32::consts::PI / num_segments as f32;
        let angle2 = ((i + 1) as f32) * 2.0 * std::f32::consts::PI / num_segments as f32;

        let p1 = Vec2::new(
            wheel_center.x + wheel_radius * angle1.cos(),
            wheel_center.y + wheel_radius * angle1.sin(),
        );
        let p2 = Vec2::new(
            wheel_center.x + wheel_radius * angle2.cos(),
            wheel_center.y + wheel_radius * angle2.sin(),
        );

        // Create color based on angle
        let hue = (i as f32) / num_segments as f32;
        let r = ((hue * 3.0).sin() * 127.5 + 127.5) as u8;
        let g = (((hue * 3.0 + 2.0) * std::f32::consts::PI / 3.0).sin() * 127.5 + 127.5) as u8;
        let b = (((hue * 3.0 + 4.0) * std::f32::consts::PI / 3.0).sin() * 127.5 + 127.5) as u8;
        let color = Color::RGBA(r, g, b, 255);

        renderer.fill_triangle_colored(
            wheel_center,
            p1,
            p2,
            Color::WHITE,
            color,
            color,
            Mat3::IDENTITY,
        );
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) = Window::new(
        "Barycentric Color Interpolation",
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
                } => {
                    framebuffer.resize(width as usize, height as usize);
                }
                _ => {}
            }
        }

        {
            let mut renderer = Renderer::new(&mut framebuffer);
            draw_barycentric_demos(&mut renderer);
        }

        window.present(&framebuffer)?;

        let frame_time = frame_start.elapsed();
        let target = Duration::from_nanos(1_000_000_000 / 60);
        if let Some(remaining) = target.checked_sub(frame_time) {
            std::thread::sleep(remaining);
        }
    }
}
