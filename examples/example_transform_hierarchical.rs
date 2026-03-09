use scratchpad_rs::Color;
use scratchpad_rs::framebuffer::FrameBuffer;
use scratchpad_rs::math::{Mat3, Vec2};
use scratchpad_rs::renderer::Renderer;
use scratchpad_rs::transform::TransformStack;
use scratchpad_rs::ui::Anchor;
use scratchpad_rs::window::Window;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use std::time::Duration;

const INITIAL_WINDOW_WIDTH: usize = 1280;
const INITIAL_WINDOW_HEIGHT: usize = 720;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) = Window::new(
        "lesson_3_2 - Hierarchical Transformations (Pivot Points & Anchors)",
        INITIAL_WINDOW_WIDTH as u32,
        INITIAL_WINDOW_HEIGHT as u32,
    )?;

    let mut frame_buffer = FrameBuffer::new(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT);

    // Demo state
    let mut demo_mode = 0; // 0 = pivot rotation, 1 = pivot scale, 2 = anchors
    let mut rotation_angle = 0.0f32;
    let mut scale_factor = 1.0f32;
    let mut use_pivot = true; // Toggle between pivot and origin transforms

    // Pivot point for transforms
    let mut pivot_point = Vec2::new(400.0, 300.0);

    // Object to transform (house shape)
    let house_size = Vec2::new(80.0, 60.0);
    let house_center = Vec2::new(400.0, 300.0);
    let house_top_left = house_center - house_size * 0.5;

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

                        // Adjust rotation (Mode 0)
                        Keycode::Q => rotation_angle -= 0.1,
                        Keycode::E => rotation_angle += 0.1,

                        // Adjust scale (Mode 1)
                        Keycode::Plus | Keycode::Equals => scale_factor *= 1.1,
                        Keycode::Minus => scale_factor /= 1.1,

                        // Toggle pivot vs origin
                        Keycode::P => use_pivot = !use_pivot,

                        // Move pivot point
                        Keycode::Left => pivot_point.x -= 10.0,
                        Keycode::Right => pivot_point.x += 10.0,
                        Keycode::Up => pivot_point.y -= 10.0,
                        Keycode::Down => pivot_point.y += 10.0,

                        // Reset
                        Keycode::R => {
                            rotation_angle = 0.0;
                            scale_factor = 1.0;
                            pivot_point = Vec2::new(400.0, 300.0);
                        }

                        _ => {}
                    }
                }

                _ => {}
            }
        }

        // Render
        {
            // Get dimensions before creating renderer to avoid borrowing issues
            let fb_width = frame_buffer.width();
            let fb_height = frame_buffer.height();

            let mut renderer = Renderer::new(&mut frame_buffer);
            renderer.clear(Color::RGBA(15, 15, 25, 255));

            match demo_mode {
                0 => draw_rotation_demo(
                    &mut renderer,
                    rotation_angle,
                    pivot_point,
                    house_top_left,
                    house_size,
                    use_pivot,
                ),
                1 => draw_scale_demo(
                    &mut renderer,
                    scale_factor,
                    pivot_point,
                    house_top_left,
                    house_size,
                    use_pivot,
                ),
                2 => draw_anchor_demo(&mut renderer, fb_width, fb_height),
                _ => {}
            }

            // Draw UI info
            draw_ui_info(
                &mut renderer,
                demo_mode,
                use_pivot,
                pivot_point,
                rotation_angle,
                scale_factor,
            );
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

fn draw_rotation_demo(
    renderer: &mut Renderer,
    angle: f32,
    pivot: Vec2,
    house_top_left: Vec2,
    house_size: Vec2,
    use_pivot: bool,
) {
    // Draw coordinate axes
    let center = Vec2::new(400.0, 300.0);
    renderer.draw_line_aa(
        Vec2::new(center.x - 200.0, center.y),
        Vec2::new(center.x + 200.0, center.y),
        Color::RGBA(60, 60, 80, 255),
        Mat3::IDENTITY,
    );
    renderer.draw_line_aa(
        Vec2::new(center.x, center.y - 200.0),
        Vec2::new(center.x, center.y + 200.0),
        Color::RGBA(60, 60, 80, 255),
        Mat3::IDENTITY,
    );

    // Draw pivot point indicator
    draw_pivot_indicator(renderer, pivot);

    // Draw house shape
    let house_points = create_house_shape(house_top_left, house_size);

    if use_pivot {
        // Rotate around pivot point
        let mut transform = TransformStack::new();
        transform.push(Mat3::rotate_around_point(angle, pivot));
        let matrix = transform.current();

        // Draw transformed house
        draw_transformed_house(renderer, &house_points, matrix, Color::CYAN);

        // Draw line from pivot to house center
        let house_center = house_top_left + house_size * 0.5;
        let transformed_center = matrix.transform_vec2(house_center);
        renderer.draw_line_aa(pivot, transformed_center, Color::YELLOW, Mat3::IDENTITY);
    } else {
        // Rotate around house origin (top-left of house)
        let house_origin = house_top_left;
        let mut transform = TransformStack::new();
        transform.push(Mat3::rotate_around_point(angle, house_origin));
        let matrix = transform.current();

        // Draw transformed house
        draw_transformed_house(renderer, &house_points, matrix, Color::MAGENTA);
    }

    // Draw original house outline (faded)
    draw_house_outline(renderer, &house_points, Color::RGBA(100, 100, 100, 100));
}

fn draw_scale_demo(
    renderer: &mut Renderer,
    scale: f32,
    pivot: Vec2,
    house_top_left: Vec2,
    house_size: Vec2,
    use_pivot: bool,
) {
    // Draw coordinate axes
    let center = Vec2::new(400.0, 300.0);
    renderer.draw_line_aa(
        Vec2::new(center.x - 200.0, center.y),
        Vec2::new(center.x + 200.0, center.y),
        Color::RGBA(60, 60, 80, 255),
        Mat3::IDENTITY,
    );
    renderer.draw_line_aa(
        Vec2::new(center.x, center.y - 200.0),
        Vec2::new(center.x, center.y + 200.0),
        Color::RGBA(60, 60, 80, 255),
        Mat3::IDENTITY,
    );

    // Draw pivot point indicator
    draw_pivot_indicator(renderer, pivot);

    // Draw house shape
    let house_points = create_house_shape(house_top_left, house_size);

    if use_pivot {
        // Scale around pivot point
        let mut transform = TransformStack::new();
        transform.push(Mat3::scale_around_point(scale, scale, pivot));
        let matrix = transform.current();

        // Draw transformed house
        draw_transformed_house(renderer, &house_points, matrix, Color::CYAN);

        // Draw line from pivot to house corner
        let house_corner = house_top_left;
        let transformed_corner = matrix.transform_vec2(house_corner);
        renderer.draw_line_aa(pivot, transformed_corner, Color::YELLOW, Mat3::IDENTITY);
    } else {
        // Scale around house origin (top-left of house)
        let house_origin = house_top_left;
        let mut transform = TransformStack::new();
        transform.push(Mat3::scale_around_point(scale, scale, house_origin));
        let matrix = transform.current();

        // Draw transformed house
        draw_transformed_house(renderer, &house_points, matrix, Color::MAGENTA);
    }

    // Draw original house outline (faded)
    draw_house_outline(renderer, &house_points, Color::RGBA(100, 100, 100, 100));
}

fn draw_anchor_demo(renderer: &mut Renderer, width: usize, height: usize) {
    let screen_width = width as f32;
    let screen_height = height as f32;

    // Draw a grid of buttons positioned using different anchors
    let button_size = Vec2::new(120.0, 40.0);
    let spacing = 20.0;

    // Top row - Top anchors
    let top_y = 100.0;
    let anchors_top = [Anchor::TopLeft, Anchor::TopCenter, Anchor::TopRight];
    let positions_top = [
        Vec2::new(spacing, top_y),
        Vec2::new(screen_width / 2.0, top_y),
        Vec2::new(screen_width - spacing, top_y),
    ];

    for (anchor, &pos) in anchors_top.iter().zip(positions_top.iter()) {
        let top_left = anchor.top_left_for(pos, button_size.x, button_size.y);
        draw_button(renderer, top_left, button_size, Color::CYAN);
        draw_anchor_indicator(renderer, pos, anchor);
    }

    // Middle row - Center anchors
    let center_y = screen_height / 2.0;
    let anchors_center = [Anchor::CenterLeft, Anchor::Center, Anchor::CenterRight];
    let positions_center = [
        Vec2::new(spacing, center_y),
        Vec2::new(screen_width / 2.0, center_y),
        Vec2::new(screen_width - spacing, center_y),
    ];

    for (anchor, &pos) in anchors_center.iter().zip(positions_center.iter()) {
        let top_left = anchor.top_left_for(pos, button_size.x, button_size.y);
        draw_button(renderer, top_left, button_size, Color::GREEN);
        draw_anchor_indicator(renderer, pos, anchor);
    }

    // Bottom row - Bottom anchors
    let bottom_y = screen_height - 100.0;
    let anchors_bottom = [
        Anchor::BottomLeft,
        Anchor::BottomCenter,
        Anchor::BottomRight,
    ];
    let positions_bottom = [
        Vec2::new(spacing, bottom_y),
        Vec2::new(screen_width / 2.0, bottom_y),
        Vec2::new(screen_width - spacing, bottom_y),
    ];

    for (anchor, &pos) in anchors_bottom.iter().zip(positions_bottom.iter()) {
        let top_left = anchor.top_left_for(pos, button_size.x, button_size.y);
        draw_button(renderer, top_left, button_size, Color::YELLOW);
        draw_anchor_indicator(renderer, pos, anchor);
    }
}

fn create_house_shape(top_left: Vec2, size: Vec2) -> Vec<Vec2> {
    vec![
        Vec2::new(top_left.x, top_left.y + size.y), // Bottom left
        Vec2::new(top_left.x + size.x, top_left.y + size.y), // Bottom right
        Vec2::new(top_left.x + size.x, top_left.y), // Top right
        Vec2::new(top_left.x + size.x / 2.0, top_left.y - size.y * 0.3), // Roof peak
        Vec2::new(top_left.x, top_left.y),          // Top left
    ]
}

fn draw_transformed_house(renderer: &mut Renderer, points: &[Vec2], matrix: Mat3, color: Color) {
    let transformed: Vec<Vec2> = points.iter().map(|&p| matrix.transform_vec2(p)).collect();

    for i in 0..transformed.len() {
        let start = transformed[i];
        let end = transformed[(i + 1) % transformed.len()];
        renderer.draw_line_aa(start, end, color, Mat3::IDENTITY);
    }
}

fn draw_house_outline(renderer: &mut Renderer, points: &[Vec2], color: Color) {
    for i in 0..points.len() {
        let start = points[i];
        let end = points[(i + 1) % points.len()];
        renderer.draw_line_aa(start, end, color, Mat3::IDENTITY);
    }
}

fn draw_pivot_indicator(renderer: &mut Renderer, pivot: Vec2) {
    // Draw crosshair at pivot
    let size = 15.0;
    renderer.draw_line_aa(
        Vec2::new(pivot.x - size, pivot.y),
        Vec2::new(pivot.x + size, pivot.y),
        Color::YELLOW,
        Mat3::IDENTITY,
    );
    renderer.draw_line_aa(
        Vec2::new(pivot.x, pivot.y - size),
        Vec2::new(pivot.x, pivot.y + size),
        Color::YELLOW,
        Mat3::IDENTITY,
    );

    // Draw circle around pivot
    renderer.draw_circle(pivot, size * 1.5, Color::YELLOW, Mat3::IDENTITY);
}

fn draw_button(renderer: &mut Renderer, top_left: Vec2, size: Vec2, color: Color) {
    let bottom_right = Vec2::new(top_left.x + size.x, top_left.y + size.y);
    renderer.fill_rect(top_left, bottom_right, color, Mat3::IDENTITY);
    renderer.draw_rect(top_left, bottom_right, Color::WHITE, Mat3::IDENTITY);
}

fn draw_anchor_indicator(renderer: &mut Renderer, anchor_point: Vec2, _anchor: &Anchor) {
    // Draw a small dot at the anchor point
    let dot_size = 5.0;
    renderer.fill_rect(
        Vec2::new(anchor_point.x - dot_size, anchor_point.y - dot_size),
        Vec2::new(anchor_point.x + dot_size, anchor_point.y + dot_size),
        Color::RED,
        Mat3::IDENTITY,
    );
}

fn draw_ui_info(
    renderer: &mut Renderer,
    demo_mode: usize,
    use_pivot: bool,
    _pivot: Vec2,
    _angle: f32,
    _scale: f32,
) {
    // Draw info panel in top-left
    let panel_x = 10.0;
    let panel_y = 10.0;
    let panel_width = 350.0;
    let panel_height = 200.0;

    renderer.fill_rect(
        Vec2::new(panel_x, panel_y),
        Vec2::new(panel_x + panel_width, panel_y + panel_height),
        Color::RGBA(0, 0, 0, 200),
        Mat3::IDENTITY,
    );

    // Draw mode indicator
    let mode_colors = [Color::CYAN, Color::GREEN, Color::YELLOW];
    let mode_color = mode_colors[demo_mode.min(2)];

    // Visual mode indicator (colored box)
    renderer.fill_rect(
        Vec2::new(panel_x + 10.0, panel_y + 10.0),
        Vec2::new(panel_x + 40.0, panel_y + 40.0),
        mode_color,
        Mat3::IDENTITY,
    );

    // Draw pivot indicator in info
    if demo_mode < 2 {
        let pivot_indicator_x = panel_x + 10.0;
        let pivot_indicator_y = panel_y + 60.0;
        let pivot_color = if use_pivot {
            Color::YELLOW
        } else {
            Color::MAGENTA
        };

        // Small pivot indicator
        renderer.fill_rect(
            Vec2::new(pivot_indicator_x, pivot_indicator_y),
            Vec2::new(pivot_indicator_x + 20.0, pivot_indicator_y + 20.0),
            pivot_color,
            Mat3::IDENTITY,
        );
    }

    // Draw controls help in bottom-right
    let help_x = (renderer.width() as f32) - 350.0;
    let help_y = (renderer.height() as f32) - 180.0;
    let help_width = 340.0;
    let help_height = 170.0;

    renderer.fill_rect(
        Vec2::new(help_x, help_y),
        Vec2::new(help_x + help_width, help_y + help_height),
        Color::RGBA(0, 0, 0, 200),
        Mat3::IDENTITY,
    );

    // Visual indicators for controls
    let controls_start_y = help_y + 20.0;
    let controls_spacing = 25.0;

    // Mode switching
    draw_control_indicator(renderer, help_x + 10.0, controls_start_y, Color::CYAN);
    draw_control_indicator(
        renderer,
        help_x + 10.0,
        controls_start_y + controls_spacing,
        Color::GREEN,
    );
    draw_control_indicator(
        renderer,
        help_x + 10.0,
        controls_start_y + controls_spacing * 2.0,
        Color::YELLOW,
    );

    if demo_mode < 2 {
        // Rotation/Scale controls
        draw_control_indicator(
            renderer,
            help_x + 10.0,
            controls_start_y + controls_spacing * 3.0,
            Color::WHITE,
        );

        // Pivot toggle
        draw_control_indicator(
            renderer,
            help_x + 10.0,
            controls_start_y + controls_spacing * 4.0,
            if use_pivot {
                Color::YELLOW
            } else {
                Color::MAGENTA
            },
        );

        // Arrow keys for pivot movement
        draw_control_indicator(
            renderer,
            help_x + 10.0,
            controls_start_y + controls_spacing * 5.0,
            Color::WHITE,
        );
    }
}

fn draw_control_indicator(renderer: &mut Renderer, x: f32, y: f32, color: Color) {
    // Draw a small colored square as visual indicator
    renderer.fill_rect(
        Vec2::new(x, y),
        Vec2::new(x + 15.0, y + 15.0),
        color,
        Mat3::IDENTITY,
    );
}
