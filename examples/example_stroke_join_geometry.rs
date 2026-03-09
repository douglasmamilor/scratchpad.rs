//! Lesson 4.3 - Join Geometry: Right then Up Turn
//!
//! This lesson demonstrates what happens at a corner when stroking a polyline.
//! Specifically, it shows a CCW turn: going right then turning up.
//!
//! Key concepts:
//! - The bottom edges of the two segments split apart (inner corner)
//! - The top edges intersect and the filled quads overlap there
//! - The outer gap (where top edges don't meet) is what the join fills
//! - The inner corner is already filled by quad overlap, not by join geometry
//!
//! Run with: `cargo run --example lesson_4_3_join_geometry`

use scratchpad_rs::Color;
use scratchpad_rs::framebuffer::FrameBuffer;
use scratchpad_rs::math::{Mat3, Point2, Vec2};
use scratchpad_rs::renderer::{LineCap, LineJoin, PolyLine, Renderer, StrokeStyle};
use scratchpad_rs::window::Window;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use std::time::Duration;

const INITIAL_WINDOW_WIDTH: usize = 1000;
const INITIAL_WINDOW_HEIGHT: usize = 800;

/// Creates a simple L-shaped polyline: right then up (CCW turn)
fn right_then_up_polyline() -> PolyLine {
    PolyLine::new(
        vec![
            Point2::new(100.0, 400.0), // point a
            Point2::new(400.0, 400.0), // point b (corner)
            Point2::new(400.0, 100.0), // point c
        ],
        false,
    )
}

fn draw_reference(renderer: &mut Renderer, poly: &PolyLine, model: Mat3) {
    let pts = poly.points();
    let red = Color::RED;
    for w in pts.windows(2) {
        renderer.draw_line_aa(w[0], w[1], red, model);
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

fn draw_points(renderer: &mut Renderer, poly: &PolyLine, model: Mat3) {
    let pts = poly.points();
    let dot_radius = 6.0;
    for pt in pts {
        let pt_transformed = model.transform_vec2(*pt);
        renderer.fill_circle(pt_transformed, dot_radius, Color::RED, Mat3::IDENTITY);
    }
}

fn draw_segment_quad_vertices(
    renderer: &mut Renderer,
    start: Point2,
    end: Point2,
    thickness: f32,
    model: Mat3,
) {
    let u = Vec2::from(end - start);
    let u_len = u.len();
    if u_len <= 1e-6 {
        return;
    }

    let u_hat = u / u_len;
    let n = Vec2::new(-u_hat.y, u_hat.x); // perp_left
    let half = thickness / 2.0;
    let offset = n * half;

    let start_t = model.transform_vec2(start);
    let end_t = model.transform_vec2(end);

    // Calculate the four quad vertices (same as stroke_segment_core)
    let p0 = start_t + offset;
    let p1 = start_t - offset;
    let p2 = end_t - offset;
    let p3 = end_t + offset;

    // Draw all four vertices as colored dots
    let dot_radius = 6.0;
    renderer.fill_circle(p0, dot_radius, Color::GREEN, Mat3::IDENTITY);
    renderer.fill_circle(p1, dot_radius, Color::BLUE, Mat3::IDENTITY);
    renderer.fill_circle(p2, dot_radius, Color::YELLOW, Mat3::IDENTITY);
    renderer.fill_circle(p3, dot_radius, Color::MAGENTA, Mat3::IDENTITY);
}

fn draw_bevel_triangle_outline(
    renderer: &mut Renderer,
    poly: &PolyLine,
    thickness: f32,
    model: Mat3,
) {
    let pts = poly.points();
    if pts.len() < 3 {
        return;
    }

    // Get the corner point b and adjacent points a, c
    let a = pts[0];
    let b = pts[1];
    let c = pts[2];

    // Calculate exactly as the join code does (matching expand.rs logic)
    let a_w = model.transform_vec2(a);
    let b_w = model.transform_vec2(b);
    let c_w = model.transform_vec2(c);

    let u_in = b_w - a_w;
    let u_out = c_w - b_w;
    let u_in_len = u_in.len();
    let u_out_len = u_out.len();

    if u_in_len <= 1e-6 || u_out_len <= 1e-6 {
        return;
    }

    let u_in_hat = u_in / u_in_len;
    let u_out_hat = u_out / u_out_len;

    // Match the exact logic from expand.rs
    let y_down = true; // Screen space
    let cross = u_in_hat.cross(u_out_hat);
    let ccw = if y_down { cross < 0.0 } else { cross > 0.0 };

    let n_in = Vec2::new(-u_in_hat.y, u_in_hat.x);
    let n_out = Vec2::new(-u_out_hat.y, u_out_hat.x);

    // Match the EXACT current code logic from expand.rs line 134-138
    let (n_outer_in, n_outer_out, _) = if ccw {
        (n_in, n_out, true)
    } else {
        (-n_in, -n_out, false)
    };

    let half = thickness / 2.0;
    let p_in_outer = b_w + n_outer_in * half;
    let p_out_outer = b_w + n_outer_out * half;

    // Draw the bevel triangle outline (what actually gets filled)
    renderer.draw_line_aa(b_w, p_in_outer, Color::BLACK, Mat3::IDENTITY);
    renderer.draw_line_aa(b_w, p_out_outer, Color::BLACK, Mat3::IDENTITY);
    renderer.draw_line_aa(p_in_outer, p_out_outer, Color::BLACK, Mat3::IDENTITY);

    // Draw the three vertices as black dots
    let dot_radius = 8.0;
    renderer.fill_circle(b_w, dot_radius, Color::BLACK, Mat3::IDENTITY);
    renderer.fill_circle(p_in_outer, dot_radius, Color::BLACK, Mat3::IDENTITY);
    renderer.fill_circle(p_out_outer, dot_radius, Color::BLACK, Mat3::IDENTITY);
}

fn draw_demo(renderer: &mut Renderer) {
    renderer.clear(Color::RGB(50, 50, 55));

    let poly = right_then_up_polyline();
    let thickness = 40.0;

    // Single example: Right then up turn with bevel join
    draw_with_join(
        renderer,
        &poly,
        LineJoin::Bevel,
        Color::CYAN,
        thickness,
        Mat3::IDENTITY,
    );

    // Draw blue line connecting points (on top of stroke)
    draw_reference(renderer, &poly, Mat3::IDENTITY);

    // Draw red dots at polyline points
    draw_points(renderer, &poly, Mat3::IDENTITY);

    // Draw all four vertices of each segment quad
    let pts = poly.points();
    if pts.len() >= 2 {
        draw_segment_quad_vertices(renderer, pts[0], pts[1], thickness, Mat3::IDENTITY);
    }
    if pts.len() >= 3 {
        draw_segment_quad_vertices(renderer, pts[1], pts[2], thickness, Mat3::IDENTITY);
    }

    // Draw bevel triangle outline and vertices (matching expand.rs calculation)
    draw_bevel_triangle_outline(renderer, &poly, thickness, Mat3::IDENTITY);
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) = Window::new(
        "Lesson 4.3 - Join Geometry: Right then Up Turn",
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
