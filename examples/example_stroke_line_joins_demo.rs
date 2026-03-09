//! Lesson 4.3 Demo - Line Joins Showcase
//!
//! This lesson demonstrates line joins in various interesting scenarios:
//! - Different turn angles (acute, right, obtuse)
//! - All three join types (Bevel, Miter, Round)
//! - Closed shapes (polygons, stars)
//! - Miter limit behavior
//! - Different stroke thicknesses
//!
//! Run with: `cargo run --example lesson_4_3_line_joins_demo`

use scratchpad_rs::Color;
use scratchpad_rs::framebuffer::FrameBuffer;
use scratchpad_rs::math::{Mat3, Point2};
use scratchpad_rs::renderer::{LineCap, LineJoin, PolyLine, Renderer, StrokeStyle};
use scratchpad_rs::window::Window;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use std::f32::consts::PI;
use std::time::Duration;

const INITIAL_WINDOW_WIDTH: usize = 1400;
const INITIAL_WINDOW_HEIGHT: usize = 900;

/// Creates a star polyline with n points
fn star_polyline(
    center: Point2,
    outer_radius: f32,
    inner_radius: f32,
    n_points: usize,
) -> PolyLine {
    let mut points = Vec::new();
    for i in 0..n_points * 2 {
        let angle = (i as f32) * PI / (n_points as f32);
        let radius = if i % 2 == 0 {
            outer_radius
        } else {
            inner_radius
        };
        let x = center.x + radius * angle.cos();
        let y = center.y + radius * angle.sin();
        points.push(Point2::new(x, y));
    }
    PolyLine::new(points, true) // closed
}

/// Creates a regular polygon
fn polygon(center: Point2, radius: f32, n_sides: usize) -> PolyLine {
    let mut points = Vec::new();
    for i in 0..n_sides {
        let angle = (i as f32) * 2.0 * PI / (n_sides as f32) - PI / 2.0; // Start at top
        let x = center.x + radius * angle.cos();
        let y = center.y + radius * angle.sin();
        points.push(Point2::new(x, y));
    }
    PolyLine::new(points, true) // closed
}

/// Creates a zigzag with varying angles
fn angle_test_polyline() -> PolyLine {
    let start_y = 50.0;
    let mut points = vec![Point2::new(50.0, start_y)];

    // Various angles: 180° (straight), 135°, 90°, 45°, 30°, 15°
    let angles = [180.0, 135.0, 90.0, 45.0, 30.0, 15.0];
    let mut x = 150.0;

    for angle_deg in angles.iter() {
        let angle_rad = *angle_deg * PI / 180.0;
        let length = 80.0;
        x += length * angle_rad.cos();
        let y = start_y - length * angle_rad.sin();
        points.push(Point2::new(x, y));
        x += 50.0; // spacing
    }

    PolyLine::new(points, false)
}

/// Creates a spiral-like path
fn spiral_polyline() -> PolyLine {
    let mut points = Vec::new();
    let center = Point2::new(200.0, 200.0);
    let turns = 2.0;
    let max_radius = 150.0;

    for i in 0..=60 {
        let t = i as f32 / 60.0;
        let angle = t * turns * 2.0 * PI;
        let radius = t * max_radius;
        let x = center.x + radius * angle.cos();
        let y = center.y + radius * angle.sin();
        points.push(Point2::new(x, y));
    }

    PolyLine::new(points, false)
}

/// Creates a path with sharp kinks to test miter limits
fn miter_limit_test() -> PolyLine {
    PolyLine::new(
        vec![
            Point2::new(50.0, 400.0),
            Point2::new(200.0, 400.0), // going right
            Point2::new(200.0, 350.0), // slight up (170° turn)
            Point2::new(250.0, 350.0), // going right
            Point2::new(250.0, 300.0), // more up (135° turn)
            Point2::new(300.0, 300.0), // going right
            Point2::new(300.0, 250.0), // 90° turn
            Point2::new(350.0, 250.0), // going right
            Point2::new(350.0, 200.0), // 45° turn (sharp)
            Point2::new(400.0, 200.0), // going right
            Point2::new(400.0, 150.0), // 30° turn (very sharp)
        ],
        false,
    )
}

fn draw_reference(renderer: &mut Renderer, poly: &PolyLine, model: Mat3) {
    let pts = poly.points();
    let n = pts.len();
    if n < 2 {
        return;
    }

    for i in 0..n - 1 {
        renderer.draw_line_aa(pts[i], pts[i + 1], Color::DARK_GRAY, model);
    }
    if poly.is_closed() && n >= 2 {
        renderer.draw_line_aa(pts[n - 1], pts[0], Color::DARK_GRAY, model);
    }
}

fn draw_with_join(
    renderer: &mut Renderer,
    poly: &PolyLine,
    join: LineJoin,
    color: Color,
    thickness: f32,
    model: Mat3,
) {
    let style = StrokeStyle::solid_screen_px(thickness, color)
        .with_cap(LineCap::Butt)
        .with_join(join);
    renderer.stroke_polyline(poly, &style, model);
}

fn draw_demo(renderer: &mut Renderer) {
    renderer.clear(Color::RGB(40, 40, 45));

    // ========== Row 1: Angle Variations ==========
    let angle_poly = angle_test_polyline();
    let model1 = Mat3::translate(0.0, 0.0);

    draw_reference(renderer, &angle_poly, model1);
    draw_with_join(
        renderer,
        &angle_poly,
        LineJoin::Bevel,
        Color::CYAN,
        20.0,
        model1,
    );

    let model2 = Mat3::translate(0.0, 120.0);
    draw_reference(renderer, &angle_poly, model2);
    draw_with_join(
        renderer,
        &angle_poly,
        LineJoin::Miter { limit: 4.0 },
        Color::MAGENTA,
        20.0,
        model2,
    );

    let model3 = Mat3::translate(0.0, 240.0);
    draw_reference(renderer, &angle_poly, model3);
    draw_with_join(
        renderer,
        &angle_poly,
        LineJoin::Round,
        Color::YELLOW,
        20.0,
        model3,
    );

    // ========== Row 2: Closed Shapes - Triangle ==========
    let triangle = polygon(Point2::new(200.0, 500.0), 80.0, 3);
    draw_reference(renderer, &triangle, Mat3::IDENTITY);
    draw_with_join(
        renderer,
        &triangle,
        LineJoin::Bevel,
        Color::CYAN,
        18.0,
        Mat3::IDENTITY,
    );

    let model_t2 = Mat3::translate(200.0, 0.0);
    draw_reference(renderer, &triangle, model_t2);
    draw_with_join(
        renderer,
        &triangle,
        LineJoin::Miter { limit: 5.0 },
        Color::MAGENTA,
        18.0,
        model_t2,
    );

    let model_t3 = Mat3::translate(400.0, 0.0);
    draw_reference(renderer, &triangle, model_t3);
    draw_with_join(
        renderer,
        &triangle,
        LineJoin::Round,
        Color::YELLOW,
        18.0,
        model_t3,
    );

    // ========== Row 3: Closed Shapes - Square ==========
    let square = polygon(Point2::new(200.0, 650.0), 80.0, 4);
    draw_reference(renderer, &square, Mat3::IDENTITY);
    draw_with_join(
        renderer,
        &square,
        LineJoin::Bevel,
        Color::CYAN,
        18.0,
        Mat3::IDENTITY,
    );

    let model_sq2 = Mat3::translate(200.0, 0.0);
    draw_reference(renderer, &square, model_sq2);
    draw_with_join(
        renderer,
        &square,
        LineJoin::Miter { limit: 5.0 },
        Color::MAGENTA,
        18.0,
        model_sq2,
    );

    let model_sq3 = Mat3::translate(400.0, 0.0);
    draw_reference(renderer, &square, model_sq3);
    draw_with_join(
        renderer,
        &square,
        LineJoin::Round,
        Color::YELLOW,
        18.0,
        model_sq3,
    );

    // ========== Row 4: Star (5-pointed) ==========
    let star = star_polyline(Point2::new(200.0, 800.0), 70.0, 35.0, 5);
    draw_reference(renderer, &star, Mat3::IDENTITY);
    draw_with_join(
        renderer,
        &star,
        LineJoin::Bevel,
        Color::CYAN,
        16.0,
        Mat3::IDENTITY,
    );

    let model_st2 = Mat3::translate(200.0, 0.0);
    draw_reference(renderer, &star, model_st2);
    draw_with_join(
        renderer,
        &star,
        LineJoin::Miter { limit: 3.0 },
        Color::MAGENTA,
        16.0,
        model_st2,
    );

    let model_st3 = Mat3::translate(400.0, 0.0);
    draw_reference(renderer, &star, model_st3);
    draw_with_join(
        renderer,
        &star,
        LineJoin::Round,
        Color::YELLOW,
        16.0,
        model_st3,
    );

    // ========== Row 5: Spiral ==========
    let spiral = spiral_polyline();
    let model_s1 = Mat3::translate(500.0, 50.0);
    draw_reference(renderer, &spiral, model_s1);
    draw_with_join(
        renderer,
        &spiral,
        LineJoin::Bevel,
        Color::CYAN,
        12.0,
        model_s1,
    );

    let model_s2 = Mat3::translate(500.0, 180.0);
    draw_reference(renderer, &spiral, model_s2);
    draw_with_join(
        renderer,
        &spiral,
        LineJoin::Miter { limit: 4.0 },
        Color::MAGENTA,
        12.0,
        model_s2,
    );

    let model_s3 = Mat3::translate(500.0, 310.0);
    draw_reference(renderer, &spiral, model_s3);
    draw_with_join(
        renderer,
        &spiral,
        LineJoin::Round,
        Color::YELLOW,
        12.0,
        model_s3,
    );

    // ========== Row 6: Miter Limit Test ==========
    let miter_test = miter_limit_test();
    let model_m1 = Mat3::translate(500.0, 450.0);
    draw_reference(renderer, &miter_test, model_m1);
    draw_with_join(
        renderer,
        &miter_test,
        LineJoin::Bevel,
        Color::CYAN,
        24.0,
        model_m1,
    );

    // Low miter limit - should clip sharp corners
    let model_m2 = Mat3::translate(500.0, 580.0);
    draw_reference(renderer, &miter_test, model_m2);
    draw_with_join(
        renderer,
        &miter_test,
        LineJoin::Miter { limit: 2.0 },
        Color::MAGENTA,
        24.0,
        model_m2,
    );

    // High miter limit - should show spikes
    let model_m3 = Mat3::translate(500.0, 710.0);
    draw_reference(renderer, &miter_test, model_m3);
    draw_with_join(
        renderer,
        &miter_test,
        LineJoin::Miter { limit: 10.0 },
        Color::from_u32(0xFF00FF00), // Green
        24.0,
        model_m3,
    );

    // Round for comparison
    let model_m4 = Mat3::translate(500.0, 840.0);
    draw_reference(renderer, &miter_test, model_m4);
    draw_with_join(
        renderer,
        &miter_test,
        LineJoin::Round,
        Color::YELLOW,
        24.0,
        model_m4,
    );

    // ========== Row 7: Hexagon ==========
    let hexagon = polygon(Point2::new(200.0, 550.0), 90.0, 6);
    draw_reference(renderer, &hexagon, Mat3::IDENTITY);
    draw_with_join(
        renderer,
        &hexagon,
        LineJoin::Bevel,
        Color::CYAN,
        20.0,
        Mat3::IDENTITY,
    );

    let model_h2 = Mat3::translate(200.0, 0.0);
    draw_reference(renderer, &hexagon, model_h2);
    draw_with_join(
        renderer,
        &hexagon,
        LineJoin::Miter { limit: 5.0 },
        Color::MAGENTA,
        20.0,
        model_h2,
    );

    let model_h3 = Mat3::translate(400.0, 0.0);
    draw_reference(renderer, &hexagon, model_h3);
    draw_with_join(
        renderer,
        &hexagon,
        LineJoin::Round,
        Color::YELLOW,
        20.0,
        model_h3,
    );

    // ========== Row 8: Octagon ==========
    let octagon = polygon(Point2::new(200.0, 750.0), 90.0, 8);
    draw_reference(renderer, &octagon, Mat3::IDENTITY);
    draw_with_join(
        renderer,
        &octagon,
        LineJoin::Bevel,
        Color::CYAN,
        20.0,
        Mat3::IDENTITY,
    );

    let model_o2 = Mat3::translate(200.0, 0.0);
    draw_reference(renderer, &octagon, model_o2);
    draw_with_join(
        renderer,
        &octagon,
        LineJoin::Miter { limit: 5.0 },
        Color::MAGENTA,
        20.0,
        model_o2,
    );

    let model_o3 = Mat3::translate(400.0, 0.0);
    draw_reference(renderer, &octagon, model_o3);
    draw_with_join(
        renderer,
        &octagon,
        LineJoin::Round,
        Color::YELLOW,
        20.0,
        model_o3,
    );

    // ========== Right side: Thick strokes ==========
    let thick_square = polygon(Point2::new(900.0, 200.0), 60.0, 4);
    draw_reference(renderer, &thick_square, Mat3::IDENTITY);
    draw_with_join(
        renderer,
        &thick_square,
        LineJoin::Bevel,
        Color::CYAN,
        30.0,
        Mat3::IDENTITY,
    );

    let model_ts2 = Mat3::translate(0.0, 150.0);
    draw_reference(renderer, &thick_square, model_ts2);
    draw_with_join(
        renderer,
        &thick_square,
        LineJoin::Miter { limit: 5.0 },
        Color::MAGENTA,
        30.0,
        model_ts2,
    );

    let model_ts3 = Mat3::translate(0.0, 300.0);
    draw_reference(renderer, &thick_square, model_ts3);
    draw_with_join(
        renderer,
        &thick_square,
        LineJoin::Round,
        Color::YELLOW,
        30.0,
        model_ts3,
    );

    // Thin strokes
    let thin_star = star_polyline(Point2::new(900.0, 600.0), 60.0, 30.0, 5);
    draw_reference(renderer, &thin_star, Mat3::IDENTITY);
    draw_with_join(
        renderer,
        &thin_star,
        LineJoin::Bevel,
        Color::CYAN,
        8.0,
        Mat3::IDENTITY,
    );

    let model_tst2 = Mat3::translate(0.0, 150.0);
    draw_reference(renderer, &thin_star, model_tst2);
    draw_with_join(
        renderer,
        &thin_star,
        LineJoin::Miter { limit: 5.0 },
        Color::MAGENTA,
        8.0,
        model_tst2,
    );

    let model_tst3 = Mat3::translate(0.0, 300.0);
    draw_reference(renderer, &thin_star, model_tst3);
    draw_with_join(
        renderer,
        &thin_star,
        LineJoin::Round,
        Color::YELLOW,
        8.0,
        model_tst3,
    );
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) = Window::new(
        "Lesson 4.3 Demo - Line Joins Showcase",
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
                } => framebuffer.resize(width as usize, height as usize),
                _ => {}
            }
        }

        {
            let mut renderer = Renderer::new(&mut framebuffer);
            draw_demo(&mut renderer);
        }

        window.present(&framebuffer)?;

        let frame_time = frame_start.elapsed();
        let target = Duration::from_nanos(1_000_000_000 / 60);
        if let Some(remaining) = target.checked_sub(frame_time) {
            std::thread::sleep(remaining);
        }
    }
}
