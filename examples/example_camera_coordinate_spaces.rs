use scratchpad_rs::Color;
use scratchpad_rs::camera::Camera;
use scratchpad_rs::framebuffer::FrameBuffer;
use scratchpad_rs::math::{Mat3, Vec2};
use scratchpad_rs::math::{Point2, Rect};
use scratchpad_rs::renderer::Renderer;
use scratchpad_rs::window::Window;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use std::time::Duration;

const INITIAL_WINDOW_WIDTH: usize = 1280;
const INITIAL_WINDOW_HEIGHT: usize = 720;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) = Window::new(
        "lesson_3_1 - Camera & Coordinate Spaces",
        INITIAL_WINDOW_WIDTH as u32,
        INITIAL_WINDOW_HEIGHT as u32,
    )?;

    let mut frame_buffer = FrameBuffer::new(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT);
    let viewport = Rect::new(
        0.0,
        0.0,
        INITIAL_WINDOW_WIDTH as f32,
        INITIAL_WINDOW_HEIGHT as f32,
    );
    let mut camera = Camera::default(viewport);

    // Mouse position tracking
    let mut mouse_screen_pos = Point2::new(0.0, 0.0);
    let mut mouse_world_pos = Point2::new(0.0, 0.0);

    loop {
        let frame_start = std::time::Instant::now();

        // Handle events
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
                } => break,

                Event::Window {
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => {
                    frame_buffer.resize(w as usize, h as usize);
                    let new_viewport = Rect::new(0.0, 0.0, w as f32, h as f32);
                    camera.set_viewport(new_viewport);
                }

                Event::MouseMotion { x, y, .. } => {
                    mouse_screen_pos = Point2::new(x as f32, y as f32);
                    mouse_world_pos = camera.screen_to_world(mouse_screen_pos);
                }

                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    match key {
                        // Camera movement - Arrow keys
                        Keycode::Left => camera.translate(Point2::new(-10.0, 0.0)),
                        Keycode::Right => camera.translate(Point2::new(10.0, 0.0)),
                        Keycode::Up => camera.translate(Point2::new(0.0, -10.0)),
                        Keycode::Down => camera.translate(Point2::new(0.0, 10.0)),

                        // Camera zoom - +/- keys
                        Keycode::Equals | Keycode::Plus => camera.zoom_by(1.1),
                        Keycode::Minus => camera.zoom_by(0.9),

                        // Camera rotation - Q/E keys
                        Keycode::Q => camera.rotate(0.1),
                        Keycode::E => camera.rotate(-0.1),

                        // Reset camera
                        Keycode::R => {
                            camera = Camera::default(camera.viewport());
                        }

                        _ => {}
                    }
                }

                _ => {}
            }
        }

        // Render
        {
            let mut renderer = Renderer::new(&mut frame_buffer);
            renderer.clear(Color::RGBA(15, 15, 25, 255));

            // Get camera view matrix for rendering
            let view_matrix = camera.view_matrix();

            // Draw world grid
            draw_world_grid(&mut renderer, &camera, &view_matrix);

            // Draw world objects
            draw_world_objects(&mut renderer, &camera, &view_matrix);

            // Draw coordinate system info
            draw_coordinate_info(&mut renderer, &camera, mouse_screen_pos, mouse_world_pos);

            // Draw camera controls help
            draw_controls_help(&mut renderer);
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

fn draw_world_grid(renderer: &mut Renderer, camera: &Camera, _view_matrix: &Mat3) {
    let grid_color = Color::RGBA(60, 60, 80, 255);
    let grid_major_color = Color::RGBA(100, 100, 120, 255);

    // Convert world coordinates to screen for grid lines
    // We'll draw a grid centered around world origin

    // Find visible world bounds
    let viewport = camera.viewport();
    let top_left_world = camera.screen_to_world(Point2::new(0.0, 0.0));
    let bottom_right_world = camera.screen_to_world(Point2::new(viewport.width, viewport.height));

    // Grid spacing
    let grid_spacing = 50.0;
    let major_grid_spacing = 200.0;

    // Draw vertical lines
    let start_x = (top_left_world.x / grid_spacing).floor() * grid_spacing;
    let end_x = (bottom_right_world.x / grid_spacing).ceil() * grid_spacing;

    for x in (start_x as i32)..=(end_x as i32) {
        let x_f32 = x as f32;
        if (x_f32 % major_grid_spacing).abs() < 1.0 {
            // Major grid line
            let start_screen = camera.world_to_screen(Point2::new(x_f32, top_left_world.y));
            let end_screen = camera.world_to_screen(Point2::new(x_f32, bottom_right_world.y));
            renderer.draw_line_aa(start_screen, end_screen, grid_major_color, Mat3::IDENTITY);
        } else if (x_f32 % grid_spacing).abs() < 1.0 {
            // Minor grid line
            let start_screen = camera.world_to_screen(Point2::new(x_f32, top_left_world.y));
            let end_screen = camera.world_to_screen(Point2::new(x_f32, bottom_right_world.y));
            renderer.draw_line_aa(start_screen, end_screen, grid_color, Mat3::IDENTITY);
        }
    }

    // Draw horizontal lines
    let start_y = (top_left_world.y / grid_spacing).floor() * grid_spacing;
    let end_y = (bottom_right_world.y / grid_spacing).ceil() * grid_spacing;

    for y in (start_y as i32)..=(end_y as i32) {
        let y_f32 = y as f32;
        if (y_f32 % major_grid_spacing).abs() < 1.0 {
            // Major grid line
            let start_screen = camera.world_to_screen(Point2::new(top_left_world.x, y_f32));
            let end_screen = camera.world_to_screen(Point2::new(bottom_right_world.x, y_f32));
            renderer.draw_line_aa(start_screen, end_screen, grid_major_color, Mat3::IDENTITY);
        } else if (y_f32 % grid_spacing).abs() < 1.0 {
            // Minor grid line
            let start_screen = camera.world_to_screen(Point2::new(top_left_world.x, y_f32));
            let end_screen = camera.world_to_screen(Point2::new(bottom_right_world.x, y_f32));
            renderer.draw_line_aa(start_screen, end_screen, grid_color, Mat3::IDENTITY);
        }
    }

    // Draw world origin axes
    let origin_screen = camera.world_to_screen(Point2::new(0.0, 0.0));
    let axis_length = 100.0;
    let right_world = camera.world_to_screen(Point2::new(axis_length, 0.0));
    let up_world = camera.world_to_screen(Point2::new(0.0, axis_length));

    // X-axis (red)
    renderer.draw_line_aa(origin_screen, right_world, Color::RED, Mat3::IDENTITY);
    // Y-axis (green)
    renderer.draw_line_aa(origin_screen, up_world, Color::GREEN, Mat3::IDENTITY);
}

fn draw_world_objects(renderer: &mut Renderer, camera: &Camera, _view_matrix: &Mat3) {
    // Draw several objects at fixed world positions

    // Object 1: Red square at world (100, 100)
    let obj1_world = Point2::new(100.0, 100.0);
    let obj1_screen = camera.world_to_screen(obj1_world);
    renderer.fill_rect(
        Vec2::new(obj1_screen.x - 20.0, obj1_screen.y - 20.0),
        Vec2::new(obj1_screen.x + 20.0, obj1_screen.y + 20.0),
        Color::RED,
        Mat3::IDENTITY,
    );

    // Object 2: Green circle at world (-150, 200)
    let obj2_world = Point2::new(-150.0, 200.0);
    let obj2_screen = camera.world_to_screen(obj2_world);
    draw_circle(renderer, obj2_screen, 30.0, Color::GREEN);

    // Object 3: Blue triangle at world (200, -100)
    let obj3_world = Point2::new(200.0, -100.0);
    let obj3_screen = camera.world_to_screen(obj3_world);
    renderer.fill_triangle(
        Vec2::new(obj3_screen.x, obj3_screen.y - 25.0),
        Vec2::new(obj3_screen.x - 20.0, obj3_screen.y + 25.0),
        Vec2::new(obj3_screen.x + 20.0, obj3_screen.y + 25.0),
        Color::BLUE,
        Mat3::IDENTITY,
    );

    // Object 4: Yellow diamond at world origin
    let origin_screen = camera.world_to_screen(Point2::new(0.0, 0.0));
    renderer.fill_triangle(
        Vec2::new(origin_screen.x, origin_screen.y - 20.0),
        Vec2::new(origin_screen.x - 15.0, origin_screen.y),
        Vec2::new(origin_screen.x, origin_screen.y + 20.0),
        Color::YELLOW,
        Mat3::IDENTITY,
    );
    renderer.fill_triangle(
        Vec2::new(origin_screen.x, origin_screen.y - 20.0),
        Vec2::new(origin_screen.x + 15.0, origin_screen.y),
        Vec2::new(origin_screen.x, origin_screen.y + 20.0),
        Color::YELLOW,
        Mat3::IDENTITY,
    );

    // Object 5: Pattern of small squares
    for i in -2..=2 {
        for j in -2..=2 {
            let pattern_world = Point2::new(i as f32 * 80.0, j as f32 * 80.0 + 300.0);
            let pattern_screen = camera.world_to_screen(pattern_world);
            let color = if (i + j) % 2 == 0 {
                Color::CYAN
            } else {
                Color::MAGENTA
            };
            renderer.fill_rect(
                Vec2::new(pattern_screen.x - 15.0, pattern_screen.y - 15.0),
                Vec2::new(pattern_screen.x + 15.0, pattern_screen.y + 15.0),
                color,
                Mat3::IDENTITY,
            );
        }
    }
}

fn draw_circle(renderer: &mut Renderer, center: Point2, radius: f32, color: Color) {
    // Simple circle using fill_ellipse
    renderer.fill_ellipse(
        Vec2::new(center.x, center.y),
        radius,
        radius,
        color,
        Mat3::IDENTITY,
    );
}

fn draw_coordinate_info(
    renderer: &mut Renderer,
    camera: &Camera,
    mouse_screen: Point2,
    mouse_world: Point2,
) {
    // Draw crosshair at mouse position
    let crosshair_size = 10.0;
    renderer.draw_line_aa(
        Vec2::new(mouse_screen.x - crosshair_size, mouse_screen.y),
        Vec2::new(mouse_screen.x + crosshair_size, mouse_screen.y),
        Color::WHITE,
        Mat3::IDENTITY,
    );
    renderer.draw_line_aa(
        Vec2::new(mouse_screen.x, mouse_screen.y - crosshair_size),
        Vec2::new(mouse_screen.x, mouse_screen.y + crosshair_size),
        Color::WHITE,
        Mat3::IDENTITY,
    );

    // Draw a small indicator at world position too
    let viewport = camera.viewport();
    let mouse_world_screen = camera.world_to_screen(mouse_world);
    if mouse_world_screen.x >= 0.0
        && mouse_world_screen.x < viewport.width
        && mouse_world_screen.y >= 0.0
        && mouse_world_screen.y < viewport.height
    {
        // Only draw if visible
        renderer.fill_rect(
            Vec2::new(mouse_world_screen.x - 5.0, mouse_world_screen.y - 5.0),
            Vec2::new(mouse_world_screen.x + 5.0, mouse_world_screen.y + 5.0),
            Color::YELLOW,
            Mat3::IDENTITY,
        );
    }
}

fn draw_controls_help(_renderer: &mut Renderer) {
    // Controls help placeholder - will display text when text rendering is implemented
    // Controls: Arrow keys (pan), +/- (zoom), Q/E (rotate), R (reset)
}
