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

fn draw_flood_fill_lesson(renderer: &mut Renderer) {
    renderer.clear(Color::BLACK);

    // Title
    draw_text(
        renderer,
        "Flood Fill Algorithm Demo",
        Vec2::new(50.0, 30.0),
        Color::WHITE,
    );

    // Section 1: Basic Concept - Simple Shapes
    draw_text(
        renderer,
        "1. Basic Concept - Fill Connected Areas",
        Vec2::new(50.0, 60.0),
        Color::YELLOW,
    );

    // Draw a simple rectangle
    renderer.fill_rect(
        Vec2::new(50.0, 80.0),
        Vec2::new(150.0, 100.0),
        Color::BLUE,
        Mat3::IDENTITY,
    );
    draw_text(renderer, "Original", Vec2::new(50.0, 190.0), Color::WHITE);

    // Fill it with flood fill
    renderer.flood_fill(
        (125, 130),
        Color::RED,
        |target, _new| target == Color::BLUE,
        false,
    );
    draw_text(
        renderer,
        "After Flood Fill",
        Vec2::new(220.0, 190.0),
        Color::WHITE,
    );

    // Section 2: Boundary Detection
    draw_text(
        renderer,
        "2. Boundary Detection - Stops at Different Colors",
        Vec2::new(50.0, 220.0),
        Color::YELLOW,
    );

    // Draw a complex shape with boundaries
    renderer.fill_rect(
        Vec2::new(50.0, 240.0),
        Vec2::new(200.0, 120.0),
        Color::GREEN,
        Mat3::IDENTITY,
    );
    renderer.fill_rect(
        Vec2::new(100.0, 260.0),
        Vec2::new(100.0, 80.0),
        Color::BLUE,
        Mat3::IDENTITY,
    );
    renderer.fill_rect(
        Vec2::new(120.0, 280.0),
        Vec2::new(60.0, 40.0),
        Color::RED,
        Mat3::IDENTITY,
    );

    // Try to fill from different starting points
    renderer.flood_fill(
        (75, 300),
        Color::YELLOW,
        |target, _new| target == Color::GREEN,
        false,
    );
    renderer.flood_fill(
        (150, 300),
        Color::CYAN,
        |target, _new| target == Color::BLUE,
        false,
    );
    renderer.flood_fill(
        (150, 300),
        Color::MAGENTA,
        |target, _new| target == Color::RED,
        false,
    );

    draw_text(
        renderer,
        "Green area filled with Yellow",
        Vec2::new(50.0, 370.0),
        Color::WHITE,
    );
    draw_text(
        renderer,
        "Blue area filled with Cyan",
        Vec2::new(50.0, 385.0),
        Color::WHITE,
    );
    draw_text(
        renderer,
        "Red area filled with Magenta",
        Vec2::new(50.0, 400.0),
        Color::WHITE,
    );

    // Section 3: 4-connected vs 8-connected
    draw_text(
        renderer,
        "3. Connectivity - 4-connected vs 8-connected",
        Vec2::new(300.0, 60.0),
        Color::YELLOW,
    );

    // Draw a shape that shows the difference
    renderer.fill_rect(
        Vec2::new(300.0, 80.0),
        Vec2::new(200.0, 200.0),
        Color::GRAY,
        Mat3::IDENTITY,
    );

    // Create a diagonal line pattern
    for i in 0..20 {
        let x = 300 + i * 10;
        let y = 80 + i * 10;
        renderer.fill_rect(
            Vec2::new(x as f32, y as f32),
            Vec2::new((x + 5) as f32, (y + 5) as f32),
            Color::BLACK,
            Mat3::IDENTITY,
        );
    }

    // 4-connected fill (stops at diagonal)
    renderer.flood_fill(
        (310, 90),
        Color::RED,
        |target, _new| target == Color::GRAY,
        false,
    );
    draw_text(
        renderer,
        "4-connected",
        Vec2::new(300.0, 290.0),
        Color::WHITE,
    );

    // 8-connected fill (crosses diagonal)
    renderer.flood_fill(
        (350, 90),
        Color::BLUE,
        |target, _new| target == Color::GRAY,
        true,
    );
    draw_text(
        renderer,
        "8-connected",
        Vec2::new(450.0, 290.0),
        Color::WHITE,
    );

    // Section 4: Custom Matching Functions
    draw_text(
        renderer,
        "4. Custom Matching - Fill Only Specific Colors",
        Vec2::new(550.0, 60.0),
        Color::YELLOW,
    );

    // Draw a pattern with multiple colors
    renderer.fill_rect(
        Vec2::new(550.0, 80.0),
        Vec2::new(200.0, 200.0),
        Color::GREEN,
        Mat3::IDENTITY,
    );
    renderer.fill_rect(
        Vec2::new(600.0, 100.0),
        Vec2::new(100.0, 50.0),
        Color::BLUE,
        Mat3::IDENTITY,
    );
    renderer.fill_rect(
        Vec2::new(650.0, 120.0),
        Vec2::new(50.0, 30.0),
        Color::RED,
        Mat3::IDENTITY,
    );
    renderer.fill_rect(
        Vec2::new(600.0, 160.0),
        Vec2::new(100.0, 50.0),
        Color::BLUE,
        Mat3::IDENTITY,
    );
    renderer.fill_rect(
        Vec2::new(650.0, 180.0),
        Vec2::new(50.0, 30.0),
        Color::RED,
        Mat3::IDENTITY,
    );

    // Fill only blue areas
    renderer.flood_fill(
        (650, 125),
        Color::YELLOW,
        |target, _new| target == Color::BLUE,
        false,
    );
    renderer.flood_fill(
        (650, 185),
        Color::YELLOW,
        |target, _new| target == Color::BLUE,
        false,
    );

    draw_text(
        renderer,
        "Only Blue areas filled",
        Vec2::new(550.0, 290.0),
        Color::WHITE,
    );

    // Section 5: Interactive Demo
    draw_text(
        renderer,
        "5. Interactive Demo - Click to Fill",
        Vec2::new(50.0, 450.0),
        Color::YELLOW,
    );

    // Draw a complex interactive shape
    draw_house(renderer, Vec2::new(50.0, 470.0));
    draw_car(renderer, Vec2::new(200.0, 470.0));
    draw_tree(renderer, Vec2::new(350.0, 470.0));
    draw_flower(renderer, Vec2::new(500.0, 470.0));

    draw_text(
        renderer,
        "Hover over shapes and press SPACE to fill with random colors",
        Vec2::new(50.0, 650.0),
        Color::WHITE,
    );
}

fn draw_house(renderer: &mut Renderer, pos: Vec2) {
    // House base
    renderer.fill_rect(
        pos,
        Vec2::new(pos.x + 80.0, pos.y + 60.0),
        Color::BROWN,
        Mat3::IDENTITY,
    );
    // Roof
    renderer.fill_triangle(
        pos,
        Vec2::new(pos.x + 40.0, pos.y - 30.0),
        Vec2::new(pos.x + 80.0, pos.y),
        Color::RED,
        Mat3::IDENTITY,
    );
    // Door
    renderer.fill_rect(
        Vec2::new(pos.x + 30.0, pos.y + 20.0),
        Vec2::new(pos.x + 50.0, pos.y + 60.0),
        Color::YELLOW,
        Mat3::IDENTITY,
    );
    // Window
    renderer.fill_rect(
        Vec2::new(pos.x + 10.0, pos.y + 10.0),
        Vec2::new(pos.x + 25.0, pos.y + 25.0),
        Color::CYAN,
        Mat3::IDENTITY,
    );
}

fn draw_car(renderer: &mut Renderer, pos: Vec2) {
    // Car body
    renderer.fill_rect(
        pos,
        Vec2::new(pos.x + 100.0, pos.y + 40.0),
        Color::BLUE,
        Mat3::IDENTITY,
    );
    // Car roof
    renderer.fill_rect(
        Vec2::new(pos.x + 20.0, pos.y - 20.0),
        Vec2::new(pos.x + 80.0, pos.y),
        Color::GREEN,
        Mat3::IDENTITY,
    );
    // Wheels
    renderer.fill_circle(
        Vec2::new(pos.x + 20.0, pos.y + 40.0),
        15.0,
        Color::BLACK,
        Mat3::IDENTITY,
    );
    renderer.fill_circle(
        Vec2::new(pos.x + 80.0, pos.y + 40.0),
        15.0,
        Color::BLACK,
        Mat3::IDENTITY,
    );
    // Windshield
    renderer.fill_rect(
        Vec2::new(pos.x + 30.0, pos.y - 15.0),
        Vec2::new(pos.x + 70.0, pos.y - 5.0),
        Color::CYAN,
        Mat3::IDENTITY,
    );
}

fn draw_tree(renderer: &mut Renderer, pos: Vec2) {
    // Trunk
    renderer.fill_rect(
        Vec2::new(pos.x + 15.0, pos.y + 30.0),
        Vec2::new(pos.x + 25.0, pos.y + 60.0),
        Color::BROWN,
        Mat3::IDENTITY,
    );
    // Leaves
    renderer.fill_circle(
        Vec2::new(pos.x + 20.0, pos.y + 20.0),
        25.0,
        Color::GREEN,
        Mat3::IDENTITY,
    );
    renderer.fill_circle(
        Vec2::new(pos.x + 5.0, pos.y + 10.0),
        20.0,
        Color::GREEN,
        Mat3::IDENTITY,
    );
    renderer.fill_circle(
        Vec2::new(pos.x + 35.0, pos.y + 10.0),
        20.0,
        Color::GREEN,
        Mat3::IDENTITY,
    );
}

fn draw_flower(renderer: &mut Renderer, pos: Vec2) {
    // Center
    renderer.fill_circle(
        Vec2::new(pos.x + 20.0, pos.y + 20.0),
        8.0,
        Color::YELLOW,
        Mat3::IDENTITY,
    );
    // Petals
    renderer.fill_circle(
        Vec2::new(pos.x + 20.0, pos.y + 5.0),
        12.0,
        Color::PINK,
        Mat3::IDENTITY,
    );
    renderer.fill_circle(
        Vec2::new(pos.x + 35.0, pos.y + 20.0),
        12.0,
        Color::PINK,
        Mat3::IDENTITY,
    );
    renderer.fill_circle(
        Vec2::new(pos.x + 20.0, pos.y + 35.0),
        12.0,
        Color::PINK,
        Mat3::IDENTITY,
    );
    renderer.fill_circle(
        Vec2::new(pos.x + 5.0, pos.y + 20.0),
        12.0,
        Color::PINK,
        Mat3::IDENTITY,
    );
    // Stem
    renderer.fill_rect(
        Vec2::new(pos.x + 18.0, pos.y + 40.0),
        Vec2::new(pos.x + 22.0, pos.y + 60.0),
        Color::GREEN,
        Mat3::IDENTITY,
    );
}

fn draw_text(renderer: &mut Renderer, text: &str, pos: Vec2, color: Color) {
    // Simple text rendering using rectangles (for demo purposes)
    let mut x = pos.x;
    for (i, _) in text.chars().enumerate() {
        if i < 20 {
            // Limit text length
            renderer.fill_rect(
                Vec2::new(x, pos.y),
                Vec2::new(x + 8.0, pos.y + 12.0),
                color,
                Mat3::IDENTITY,
            );
            x += 10.0;
        }
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) = Window::new(
        "lesson_2_2_7 - Flood Fill Algorithm",
        INITIAL_WINDOW_WIDTH as u32,
        INITIAL_WINDOW_HEIGHT as u32,
    )?;

    let mut frame_buffer = FrameBuffer::new(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT);
    let mut mouse_pos = (0, 0);
    let mut fill_color = Color::RED;

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
                Event::MouseMotion { x, y, .. } => {
                    mouse_pos = (x, y);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    // Cycle through colors
                    fill_color = match fill_color {
                        Color::RED => Color::GREEN,
                        Color::GREEN => Color::BLUE,
                        Color::BLUE => Color::YELLOW,
                        Color::YELLOW => Color::MAGENTA,
                        Color::MAGENTA => Color::CYAN,
                        Color::CYAN => Color::WHITE,
                        Color::WHITE => Color::RED,
                        _ => Color::RED,
                    };
                }
                _ => {}
            }
        }

        {
            let mut renderer = Renderer::new(&mut frame_buffer);
            draw_flood_fill_lesson(&mut renderer);

            // Interactive flood fill at mouse position
            if mouse_pos.0 > 0 && mouse_pos.1 > 0 {
                renderer.flood_fill(
                    (mouse_pos.0, mouse_pos.1),
                    fill_color,
                    |target, _new| target != Color::BLACK && target != fill_color,
                    false,
                );
            }
        }

        window.present(&frame_buffer)?;

        let frame_time = frame_start.elapsed();
        let target = Duration::from_nanos(1_000_000_000 / 60);
        if let Some(remaining) = target.checked_sub(frame_time) {
            std::thread::sleep(remaining);
        }
    }
}
