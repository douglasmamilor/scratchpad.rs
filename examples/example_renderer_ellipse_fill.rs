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

fn draw_filled_ellipses(renderer: &mut Renderer) {
    renderer.clear(Color::BLACK);

    let center = Vec2::new(
        renderer.width() as f32 / 2.0,
        renderer.height() as f32 / 2.0,
    );

    // Draw a variety of filled ellipses to demonstrate the algorithm
    let colors = [
        Color::RED,
        Color::GREEN,
        Color::BLUE,
        Color::YELLOW,
        Color::MAGENTA,
        Color::CYAN,
    ];

    // 1. Concentric filled ellipses with different aspect ratios
    for (i, color) in colors.iter().enumerate() {
        let radius_x = 50 + (i as i32) * 25;
        let radius_y = 30 + (i as i32) * 15;

        renderer.fill_ellipse(
            center,
            radius_x as f32,
            radius_y as f32,
            *color,
            Mat3::IDENTITY,
        );
    }

    // 2. Horizontal filled ellipses (wide)
    for i in 0..5 {
        let x = 100 + i * 200;
        let y = 100;
        let radius_x = 80;
        let radius_y = 40;

        renderer.fill_ellipse(
            Vec2::new(x as f32, y as f32),
            radius_x as f32,
            radius_y as f32,
            Color::LIGHT_GRAY,
            Mat3::IDENTITY,
        );
    }

    // 3. Vertical filled ellipses (tall)
    for i in 0..5 {
        let x = 100 + i * 200;
        let y = 600;
        let radius_x = 40;
        let radius_y = 80;

        renderer.fill_ellipse(
            Vec2::new(x as f32, y as f32),
            radius_x as f32,
            radius_y as f32,
            Color::DARK_GRAY,
            Mat3::IDENTITY,
        );
    }

    // 4. Degenerate cases
    // Single point
    renderer.fill_ellipse(
        Vec2::new(50.0, 50.0),
        0.0,
        0.0,
        Color::WHITE,
        Mat3::IDENTITY,
    );

    // Horizontal line
    renderer.fill_ellipse(
        Vec2::new(150.0, 50.0),
        20.0,
        0.0,
        Color::WHITE,
        Mat3::IDENTITY,
    );

    // Vertical line
    renderer.fill_ellipse(
        Vec2::new(250.0, 50.0),
        0.0,
        20.0,
        Color::WHITE,
        Mat3::IDENTITY,
    );

    // 5. Circles (special case of ellipses with equal radii)
    for i in 0..3 {
        let x = 1000 + i * 80;
        let y = 200;
        let radius = 30 + i * 10;

        renderer.fill_ellipse(
            Vec2::new(x as f32, y as f32),
            radius as f32,
            radius as f32,
            Color::PINK,
            Mat3::IDENTITY,
        );
    }

    // 6. Very thin ellipses to test edge cases
    renderer.fill_ellipse(
        Vec2::new(1000.0, 400.0),
        60.0,
        5.0,
        Color::BROWN,
        Mat3::IDENTITY,
    );
    renderer.fill_ellipse(
        Vec2::new(1000.0, 450.0),
        5.0,
        60.0,
        Color::BROWN,
        Mat3::IDENTITY,
    );

    // 7. Large ellipses to test performance
    renderer.fill_ellipse(center, 200.0, 150.0, Color::GRAY, Mat3::IDENTITY);

    // 8. Outline ellipses for comparison
    renderer.draw_ellipse(
        Vec2::new(100.0, 300.0),
        60.0,
        40.0,
        Color::WHITE,
        Mat3::IDENTITY,
    );
    renderer.draw_ellipse(
        Vec2::new(200.0, 300.0),
        60.0,
        40.0,
        Color::WHITE,
        Mat3::IDENTITY,
    );

    // 9. Filled ellipses with different colors
    renderer.fill_ellipse(
        Vec2::new(100.0, 300.0),
        60.0,
        40.0,
        Color::RED,
        Mat3::IDENTITY,
    );
    renderer.fill_ellipse(
        Vec2::new(200.0, 300.0),
        60.0,
        40.0,
        Color::BLUE,
        Mat3::IDENTITY,
    );

    // 10. Semi-transparent ellipses (if supported)
    renderer.fill_ellipse(
        Vec2::new(400.0, 300.0),
        80.0,
        60.0,
        Color::RGBA(255, 0, 0, 128),
        Mat3::IDENTITY,
    ); // Semi-transparent red
    renderer.fill_ellipse(
        Vec2::new(450.0, 300.0),
        80.0,
        60.0,
        Color::RGBA(0, 0, 255, 128),
        Mat3::IDENTITY,
    ); // Semi-transparent blue

    // 11. Animated ellipses (varying radius)
    for i in 0..10 {
        let radius = 20 + i * 5;
        let color = if i % 2 == 0 {
            Color::ORANGE
        } else {
            Color::PURPLE
        };
        renderer.fill_ellipse(
            Vec2::new(600.0, 300.0),
            radius as f32,
            radius as f32,
            color,
            Mat3::IDENTITY,
        );
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) = Window::new(
        "lesson_2_4 - Filled Ellipse Drawing",
        INITIAL_WINDOW_WIDTH as u32,
        INITIAL_WINDOW_HEIGHT as u32,
    )?;

    let mut frame_buffer = FrameBuffer::new(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT);

    window.clear(0, 0, 0);

    let mut renderer = Renderer::new(&mut frame_buffer);
    draw_filled_ellipses(&mut renderer);

    _ = window.present(&frame_buffer);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::Window {
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => {
                    frame_buffer.resize(w as usize, h as usize);
                    let mut renderer = Renderer::new(&mut frame_buffer);
                    draw_filled_ellipses(&mut renderer);
                    _ = window.present(&frame_buffer);
                }
                _ => {}
            }
        }
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
