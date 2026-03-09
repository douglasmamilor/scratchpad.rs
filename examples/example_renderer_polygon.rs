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

fn draw_polygon_demos(renderer: &mut Renderer) {
    renderer.clear(Color::DARK_GRAY); // Dark background

    // Demo 1: Basic triangle using draw_polygon
    let triangle = vec![
        Vec2::new(100.0, 100.0), // Top vertex
        Vec2::new(50.0, 200.0),  // Bottom left
        Vec2::new(150.0, 200.0), // Bottom right
    ];
    renderer.draw_polygon(&triangle, Color::RED, Mat3::IDENTITY);

    // Demo 2: Square
    let square = vec![
        Vec2::new(250.0, 100.0), // Top left
        Vec2::new(350.0, 100.0), // Top right
        Vec2::new(350.0, 200.0), // Bottom right
        Vec2::new(250.0, 200.0), // Bottom left
    ];
    renderer.draw_polygon(&square, Color::GREEN, Mat3::IDENTITY);

    // Demo 3: Pentagon
    let pentagon = create_regular_polygon(Vec2::new(500.0, 150.0), 50.0, 5, 0.0);
    renderer.draw_polygon(&pentagon, Color::BLUE, Mat3::IDENTITY);

    // Demo 4: Hexagon
    let hexagon = create_regular_polygon(Vec2::new(650.0, 150.0), 50.0, 6, 0.0);
    renderer.draw_polygon(&hexagon, Color::YELLOW, Mat3::IDENTITY);

    // Demo 5: Octagon
    let octagon = create_regular_polygon(Vec2::new(800.0, 150.0), 50.0, 8, 0.0);
    renderer.draw_polygon(&octagon, Color::CYAN, Mat3::IDENTITY);

    // Demo 6: Star shape
    let star = create_star(Vec2::new(950.0, 150.0), 40.0, 20.0, 5);
    renderer.draw_polygon(&star, Color::MAGENTA, Mat3::IDENTITY);

    // Demo 7: L-shaped polygon
    let l_shape = vec![
        Vec2::new(100.0, 300.0), // Top left
        Vec2::new(200.0, 300.0), // Top right
        Vec2::new(200.0, 350.0), // Right middle
        Vec2::new(150.0, 350.0), // Inner right
        Vec2::new(150.0, 400.0), // Inner bottom
        Vec2::new(100.0, 400.0), // Bottom left
    ];
    renderer.draw_polygon(&l_shape, Color::ORANGE, Mat3::IDENTITY);

    // Demo 8: Arrow shape
    let arrow = vec![
        Vec2::new(300.0, 350.0), // Arrow tip
        Vec2::new(250.0, 330.0), // Top wing
        Vec2::new(250.0, 370.0), // Bottom wing
        Vec2::new(200.0, 370.0), // Bottom right
        Vec2::new(200.0, 380.0), // Bottom right corner
        Vec2::new(250.0, 380.0), // Bottom left
        Vec2::new(250.0, 400.0), // Bottom left corner
        Vec2::new(300.0, 400.0), // Bottom tip
    ];
    renderer.draw_polygon(&arrow, Color::GREEN, Mat3::IDENTITY);

    // Demo 9: Wireframe house
    draw_house(renderer, Vec2::new(500.0, 350.0));

    // Demo 10: Concentric circles (using many-sided polygons)
    for i in 0..5 {
        let radius = 30.0 + (i as f32) * 15.0;
        let circle = create_regular_polygon(Vec2::new(750.0, 350.0), radius, 32, 0.0);
        let color = match i {
            0 => Color::RED,
            1 => Color::GREEN,
            2 => Color::BLUE,
            3 => Color::YELLOW,
            4 => Color::CYAN,
            _ => Color::WHITE,
        };
        renderer.draw_polygon(&circle, color, Mat3::IDENTITY);
    }

    // Demo 11: Rotating triangle using draw_polygon
    let rotation = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as f32)
        * 0.001;

    let center = Vec2::new(950.0, 350.0);
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
    let rotating_triangle = vec![a, b, c];
    renderer.draw_polygon(&rotating_triangle, Color::PINK, Mat3::IDENTITY);

    // Demo 12: Edge cases
    // Single point (should do nothing)
    let single_point = vec![Vec2::new(100.0, 500.0)];
    renderer.draw_polygon(&single_point, Color::WHITE, Mat3::IDENTITY);

    // Two points (line)
    let line = vec![Vec2::new(150.0, 500.0), Vec2::new(250.0, 500.0)];
    renderer.draw_polygon(&line, Color::WHITE, Mat3::IDENTITY);

    // Empty array (should do nothing)
    let empty: Vec<Vec2> = vec![];
    renderer.draw_polygon(&empty, Color::WHITE, Mat3::IDENTITY);
}

fn create_regular_polygon(center: Vec2, radius: f32, sides: usize, rotation: f32) -> Vec<Vec2> {
    let mut vertices = Vec::new();

    for i in 0..sides {
        let angle = rotation + (i as f32 * 2.0 * PI) / sides as f32;
        let x = center.x + radius * angle.cos();
        let y = center.y + radius * angle.sin();
        vertices.push(Vec2::new(x, y));
    }

    vertices
}

fn create_star(center: Vec2, outer_radius: f32, inner_radius: f32, points: usize) -> Vec<Vec2> {
    let mut vertices = Vec::new();

    for i in 0..points * 2 {
        let angle = (i as f32 * PI) / points as f32;
        let radius = if i % 2 == 0 {
            outer_radius
        } else {
            inner_radius
        };
        let x = center.x + radius * angle.cos();
        let y = center.y + radius * angle.sin();
        vertices.push(Vec2::new(x, y));
    }

    vertices
}

fn draw_house(renderer: &mut Renderer, position: Vec2) {
    // House base
    let house_base = vec![
        Vec2::new(position.x - 60.0, position.y - 30.0), // Bottom left
        Vec2::new(position.x + 60.0, position.y - 30.0), // Bottom right
        Vec2::new(position.x + 60.0, position.y + 30.0), // Top right
        Vec2::new(position.x - 60.0, position.y + 30.0), // Top left
    ];
    renderer.draw_polygon(&house_base, Color::WHITE, Mat3::IDENTITY);

    // Roof
    let roof = vec![
        Vec2::new(position.x - 60.0, position.y + 30.0), // Bottom left
        Vec2::new(position.x + 60.0, position.y + 30.0), // Bottom right
        Vec2::new(position.x, position.y + 60.0),        // Top peak
    ];
    renderer.draw_polygon(&roof, Color::RED, Mat3::IDENTITY);

    // Door
    let door = vec![
        Vec2::new(position.x - 15.0, position.y - 30.0), // Bottom left
        Vec2::new(position.x + 15.0, position.y - 30.0), // Bottom right
        Vec2::new(position.x + 15.0, position.y + 15.0), // Top right
        Vec2::new(position.x - 15.0, position.y + 15.0), // Top left
    ];
    renderer.draw_polygon(&door, Color::BROWN, Mat3::IDENTITY);

    // Windows
    let left_window = vec![
        Vec2::new(position.x - 45.0, position.y - 5.0), // Bottom left
        Vec2::new(position.x - 30.0, position.y - 5.0), // Bottom right
        Vec2::new(position.x - 30.0, position.y + 10.0), // Top right
        Vec2::new(position.x - 45.0, position.y + 10.0), // Top left
    ];
    renderer.draw_polygon(&left_window, Color::CYAN, Mat3::IDENTITY);

    let right_window = vec![
        Vec2::new(position.x + 30.0, position.y - 5.0), // Bottom left
        Vec2::new(position.x + 45.0, position.y - 5.0), // Bottom right
        Vec2::new(position.x + 45.0, position.y + 10.0), // Top right
        Vec2::new(position.x + 30.0, position.y + 10.0), // Top left
    ];
    renderer.draw_polygon(&right_window, Color::CYAN, Mat3::IDENTITY);
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) = Window::new(
        "lesson_2_2_3 - Polygon Drawing",
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
            draw_polygon_demos(&mut renderer);
        }

        window.present(&frame_buffer)?;

        let frame_time = frame_start.elapsed();
        let target = Duration::from_nanos(1_000_000_000 / 60);
        if let Some(remaining) = target.checked_sub(frame_time) {
            std::thread::sleep(remaining);
        }
    }
}
