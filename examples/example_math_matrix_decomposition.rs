use scratchpad_rs::Color;
use scratchpad_rs::framebuffer::FrameBuffer;
use scratchpad_rs::math::{AffineDecomposition, Decomposition, Mat3, Vec2, Vec3};
use scratchpad_rs::renderer::Renderer;
use scratchpad_rs::window::Window;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use std::time::Duration;

const INITIAL_WINDOW_WIDTH: usize = 1280;
const INITIAL_WINDOW_HEIGHT: usize = 720;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) = Window::new(
        "lesson_3_3 - Advanced Matrix Operations (Decomposition)",
        INITIAL_WINDOW_WIDTH as u32,
        INITIAL_WINDOW_HEIGHT as u32,
    )?;

    let mut frame_buffer = FrameBuffer::new(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT);

    // Demo state
    let mut demo_mode = 0; // 0 = basic decomposition, 1 = affine decomposition, 2 = round-trip
    let mut transform_params = TransformParams {
        translation: Vec2::new(100.0, 100.0),
        rotation: std::f32::consts::PI / 6.0, // 30 degrees
        scale: Vec2::new(1.5, 1.2),
        shear: Vec2::new(0.0, 0.0),
    };

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
                } => return Ok(()),

                Event::Window {
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => {
                    frame_buffer.resize(w as usize, h as usize);
                }

                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    match key {
                        // Switch demo modes
                        Keycode::Num1 => demo_mode = 0,
                        Keycode::Num2 => demo_mode = 1,
                        Keycode::Num3 => demo_mode = 2,

                        // Adjust translation
                        Keycode::Left => transform_params.translation.x -= 10.0,
                        Keycode::Right => transform_params.translation.x += 10.0,
                        Keycode::Up => transform_params.translation.y -= 10.0,
                        Keycode::Down => transform_params.translation.y += 10.0,

                        // Adjust rotation
                        Keycode::Q => transform_params.rotation -= 0.1,
                        Keycode::E => transform_params.rotation += 0.1,

                        // Adjust scale
                        Keycode::A => {
                            transform_params.scale.x = (transform_params.scale.x - 0.1).max(0.1);
                        }
                        Keycode::D => {
                            transform_params.scale.x += 0.1;
                        }
                        Keycode::W => {
                            transform_params.scale.y = (transform_params.scale.y - 0.1).max(0.1);
                        }
                        Keycode::S => {
                            transform_params.scale.y += 0.1;
                        }

                        // Adjust shear (only in affine mode)
                        Keycode::J => {
                            transform_params.shear.x = (transform_params.shear.x - 0.05).max(-1.0);
                        }
                        Keycode::L => {
                            transform_params.shear.x = (transform_params.shear.x + 0.05).min(1.0);
                        }
                        Keycode::I => {
                            transform_params.shear.y = (transform_params.shear.y - 0.05).max(-1.0);
                        }
                        Keycode::K => {
                            transform_params.shear.y = (transform_params.shear.y + 0.05).min(1.0);
                        }

                        // Reset
                        Keycode::R => {
                            transform_params = TransformParams {
                                translation: Vec2::new(100.0, 100.0),
                                rotation: std::f32::consts::PI / 6.0,
                                scale: Vec2::new(1.5, 1.2),
                                shear: Vec2::new(0.0, 0.0),
                            };
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
            renderer.clear(Color::RGBA(10, 10, 20, 255));

            match demo_mode {
                0 => draw_basic_decomposition_demo(&mut renderer, &transform_params),
                1 => draw_affine_decomposition_demo(&mut renderer, &transform_params),
                2 => draw_round_trip_demo(&mut renderer, &transform_params),
                _ => {}
            }

            // Draw UI
            draw_ui(&mut renderer, demo_mode, &transform_params);
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

struct TransformParams {
    translation: Vec2,
    rotation: f32,
    scale: Vec2,
    shear: Vec2,
}

fn draw_basic_decomposition_demo(renderer: &mut Renderer, params: &TransformParams) {
    // Create a composed matrix
    let matrix = Mat3::translate(params.translation.x, params.translation.y)
        * Mat3::rotate(params.rotation)
        * Mat3::scale(params.scale.x, params.scale.y);

    // Decompose it
    let decomp = matrix.decompose();

    // Draw coordinate axes at origin
    let origin = Vec2::new(400.0, 300.0);
    renderer.draw_line_aa(
        origin,
        Vec2::new(origin.x + 100.0, origin.y),
        Color::RED,
        Mat3::IDENTITY,
    );
    renderer.draw_line_aa(
        origin,
        Vec2::new(origin.x, origin.y + 100.0),
        Color::GREEN,
        Mat3::IDENTITY,
    );

    // Draw original shape (house) at world origin
    let house_points = vec![
        Vec2::new(-40.0, -30.0), // Bottom left
        Vec2::new(40.0, -30.0),  // Bottom right
        Vec2::new(40.0, 30.0),   // Top right
        Vec2::new(0.0, 60.0),    // Roof peak
        Vec2::new(-40.0, 30.0),  // Top left
    ];

    // Draw original shape (transformed by matrix)
    let transformed_points: Vec<Vec2> = house_points
        .iter()
        .map(|&p| {
            let p3 = Vec3::new(p.x, p.y, 1.0);
            let result = matrix * p3;
            Vec2::new(result.x, result.y)
        })
        .collect();

    // Draw transformed house
    for i in 0..transformed_points.len() {
        let start = transformed_points[i];
        let end = transformed_points[(i + 1) % transformed_points.len()];
        renderer.draw_line_aa(start, end, Color::CYAN, Mat3::IDENTITY);
    }

    // Draw decomposition visualization
    draw_decomposition_info(
        renderer,
        &decomp,
        Vec2::new(50.0, 50.0),
        "Basic Decomposition",
    );

    // Draw component arrows
    let center = Vec2::new(600.0, 400.0);

    // Translation vector
    renderer.draw_line_aa(
        center,
        Vec2::new(
            center.x + decomp.translation.x,
            center.y + decomp.translation.y,
        ),
        Color::YELLOW,
        Mat3::IDENTITY,
    );
    draw_arrow_head(
        renderer,
        center,
        Vec2::new(
            center.x + decomp.translation.x,
            center.y + decomp.translation.y,
        ),
        Color::YELLOW,
    );

    // Scale indicators
    let scale_center = Vec2::new(800.0, 300.0);
    draw_scale_indicator(renderer, scale_center, decomp.scale, Color::MAGENTA);
}

fn draw_affine_decomposition_demo(renderer: &mut Renderer, params: &TransformParams) {
    // Create matrix with shear
    let matrix = if params.shear.x.abs() > 0.001 || params.shear.y.abs() > 0.001 {
        Mat3::translate(params.translation.x, params.translation.y)
            * Mat3::rotate(params.rotation)
            * Mat3::scale(params.scale.x, params.scale.y)
            * Mat3::shear(params.shear.x, params.shear.y)
    } else {
        Mat3::translate(params.translation.x, params.translation.y)
            * Mat3::rotate(params.rotation)
            * Mat3::scale(params.scale.x, params.scale.y)
    };

    // Decompose using affine decomposition
    let decomp = matrix.decompose_affine();

    // Draw coordinate axes
    let origin = Vec2::new(400.0, 300.0);
    renderer.draw_line_aa(
        origin,
        Vec2::new(origin.x + 100.0, origin.y),
        Color::RED,
        Mat3::IDENTITY,
    );
    renderer.draw_line_aa(
        origin,
        Vec2::new(origin.x, origin.y + 100.0),
        Color::GREEN,
        Mat3::IDENTITY,
    );

    // Draw shape with transformation
    let house_points = vec![
        Vec2::new(-40.0, -30.0),
        Vec2::new(40.0, -30.0),
        Vec2::new(40.0, 30.0),
        Vec2::new(0.0, 60.0),
        Vec2::new(-40.0, 30.0),
    ];

    let transformed_points: Vec<Vec2> = house_points
        .iter()
        .map(|&p| {
            let p3 = Vec3::new(p.x, p.y, 1.0);
            let result = matrix * p3;
            Vec2::new(result.x, result.y)
        })
        .collect();

    for i in 0..transformed_points.len() {
        let start = transformed_points[i];
        let end = transformed_points[(i + 1) % transformed_points.len()];
        renderer.draw_line_aa(start, end, Color::CYAN, Mat3::IDENTITY);
    }

    // Draw affine decomposition info
    draw_affine_decomposition_info(
        renderer,
        &decomp,
        Vec2::new(50.0, 50.0),
        "Affine Decomposition (with Shear)",
    );

    // Draw shear visualization
    let center = Vec2::new(600.0, 400.0);
    if decomp.shear.x.abs() > 0.001 {
        let shear_angle = f32::atan(decomp.shear.x);
        let shear_length = 80.0;
        let end = Vec2::new(
            center.x + shear_length * shear_angle.cos(),
            center.y + shear_length * shear_angle.sin(),
        );
        renderer.draw_line_aa(center, end, Color::YELLOW, Mat3::IDENTITY);
        draw_arrow_head(renderer, center, end, Color::YELLOW);
    }
}

fn draw_round_trip_demo(renderer: &mut Renderer, params: &TransformParams) {
    // Create original matrix
    let original = Mat3::translate(params.translation.x, params.translation.y)
        * Mat3::rotate(params.rotation)
        * Mat3::scale(params.scale.x, params.scale.y);

    // Decompose
    let decomp = original.decompose();

    // Recompose
    let recomposed = Mat3::recompose(decomp);

    // Calculate difference
    let diff = (original - recomposed).frobenius_norm();

    // Draw both shapes side by side
    let left_center = Vec2::new(300.0, 350.0);
    let right_center = Vec2::new(900.0, 350.0);

    let house_points = vec![
        Vec2::new(-40.0, -30.0),
        Vec2::new(40.0, -30.0),
        Vec2::new(40.0, 30.0),
        Vec2::new(0.0, 60.0),
        Vec2::new(-40.0, 30.0),
    ];

    // Draw original (left)
    let original_points: Vec<Vec2> = house_points
        .iter()
        .map(|&p| {
            let p3 = Vec3::new(p.x, p.y, 1.0);
            let result = original * p3;
            Vec2::new(left_center.x + result.x, left_center.y + result.y)
        })
        .collect();

    for i in 0..original_points.len() {
        let start = original_points[i];
        let end = original_points[(i + 1) % original_points.len()];
        renderer.draw_line_aa(start, end, Color::CYAN, Mat3::IDENTITY);
    }

    // Draw recomposed (right)
    let recomposed_points: Vec<Vec2> = house_points
        .iter()
        .map(|&p| {
            let p3 = Vec3::new(p.x, p.y, 1.0);
            let result = recomposed * p3;
            Vec2::new(right_center.x + result.x, right_center.y + result.y)
        })
        .collect();

    for i in 0..recomposed_points.len() {
        let start = recomposed_points[i];
        let end = recomposed_points[(i + 1) % recomposed_points.len()];
        renderer.draw_line_aa(start, end, Color::YELLOW, Mat3::IDENTITY);
    }

    // Draw labels
    draw_info_panel(
        renderer,
        Vec2::new(50.0, 50.0),
        &format!(
            "Round-Trip Test\n\nOriginal Matrix (left, cyan)\nRecomposed Matrix (right, yellow)\n\nDifference: {:.6}\n\nComponents:\n  Translation: ({:.1}, {:.1})\n  Rotation: {:.2} rad ({:.1}°)\n  Scale: ({:.2}, {:.2})",
            diff,
            decomp.translation.x,
            decomp.translation.y,
            decomp.rotation,
            decomp.rotation.to_degrees(),
            decomp.scale.x,
            decomp.scale.y
        ),
    );
}

fn draw_decomposition_info(
    renderer: &mut Renderer,
    decomp: &Decomposition,
    pos: Vec2,
    title: &str,
) {
    let info = format!(
        "{}\n\nTranslation: ({:.1}, {:.1})\nRotation: {:.3} rad ({:.1}°)\nScale: ({:.2}, {:.2})",
        title,
        decomp.translation.x,
        decomp.translation.y,
        decomp.rotation,
        decomp.rotation * 180.0 / std::f32::consts::PI,
        decomp.scale.x,
        decomp.scale.y
    );
    draw_info_panel(renderer, pos, &info);
}

fn draw_affine_decomposition_info(
    renderer: &mut Renderer,
    decomp: &AffineDecomposition,
    pos: Vec2,
    title: &str,
) {
    let info = format!(
        "{}\n\nTranslation: ({:.1}, {:.1})\nRotation: {:.3} rad ({:.1}°)\nScale: ({:.2}, {:.2})\nShear: ({:.3}, {:.3})",
        title,
        decomp.translation.x,
        decomp.translation.y,
        decomp.rotation,
        decomp.rotation * 180.0 / std::f32::consts::PI,
        decomp.scale.x,
        decomp.scale.y,
        decomp.shear.x,
        decomp.shear.y
    );
    draw_info_panel(renderer, pos, &info);
}

fn draw_info_panel(_renderer: &mut Renderer, _pos: Vec2, _text: &str) {
    // Info panel placeholder - will display text when text rendering is implemented
}

fn draw_arrow_head(renderer: &mut Renderer, start: Vec2, end: Vec2, color: Color) {
    // Draw a small arrow head at the end
    let dir = (end - start).normalize_or_zero();
    let perp = Vec2::new(-dir.y, dir.x);
    let arrow_size = 8.0;

    let tip = end;
    let left = end - dir * arrow_size + perp * arrow_size * 0.5;
    let right = end - dir * arrow_size - perp * arrow_size * 0.5;

    renderer.fill_triangle(tip, left, right, color, Mat3::IDENTITY);
}

fn draw_scale_indicator(renderer: &mut Renderer, center: Vec2, scale: Vec2, color: Color) {
    // Draw scale as a rectangle
    let size = 50.0;
    let rect_size = Vec2::new(size * scale.x, size * scale.y);
    renderer.fill_rect(
        Vec2::new(center.x - rect_size.x / 2.0, center.y - rect_size.y / 2.0),
        Vec2::new(center.x + rect_size.x / 2.0, center.y + rect_size.y / 2.0),
        color,
        Mat3::IDENTITY,
    );
    renderer.draw_rect(
        Vec2::new(center.x - size / 2.0, center.y - size / 2.0),
        Vec2::new(center.x + size / 2.0, center.y + size / 2.0),
        Color::WHITE,
        Mat3::IDENTITY,
    );
}

fn draw_ui(_renderer: &mut Renderer, _demo_mode: usize, _params: &TransformParams) {
    // UI help text placeholder - will display when text rendering is implemented
    // Controls: 1/2/3 (modes), Arrow keys (translation), Q/E (rotate), A/D/W/S (scale), J/L/I/K (shear), R (reset)
}
