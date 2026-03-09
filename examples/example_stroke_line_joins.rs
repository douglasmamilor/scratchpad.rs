//! Lesson 4.3 - Line Joins (Bevel / Miter / Round)
//!
//! This lesson shows join types on long, easily visible polylines.
//! Each row uses the same long zig-zag path; only the join type changes.
//! Angles include:
//! - Near-180° flats (joins should be invisible)
//! - 90° bends (round is obvious, miter is square, bevel is chamfer)
//! - Acute kinks (miter spikes vs bevel chops)
//! Run with: `cargo run --example lesson_4_3_line_joins`

use scratchpad_rs::Color;
use scratchpad_rs::framebuffer::FrameBuffer;
use scratchpad_rs::math::{Mat3, Point2};
use scratchpad_rs::renderer::{LineCap, LineJoin, PolyLine, Renderer, StrokeStyle};
use scratchpad_rs::window::Window;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use std::time::Duration;

const INITIAL_WINDOW_WIDTH: usize = 1280;
const INITIAL_WINDOW_HEIGHT: usize = 720;

fn long_polyline() -> PolyLine {
    // Y-down screen space. Mix of gentle slopes, 90° turns, and acute kinks.
    PolyLine::new(
        vec![
            Point2::new(60.0, 260.0),
            Point2::new(260.0, 260.0),  // flat
            Point2::new(360.0, 130.0),  // sharp up-left
            Point2::new(460.0, 260.0),  // down-right
            Point2::new(620.0, 260.0),  // flat
            Point2::new(700.0, 170.0),  // mild up
            Point2::new(820.0, 310.0),  // mild down
            Point2::new(940.0, 140.0),  // acute up-left kink (shows miter spike)
            Point2::new(1120.0, 260.0), // down-right
        ],
        false,
    )
}

fn draw_row(renderer: &mut Renderer, y: f32, join: LineJoin, color: Color, thickness: f32) {
    let poly = long_polyline();
    let model = Mat3::translate(0.0, y);

    // Hairline reference of the centerline
    let pts = poly.points();
    for w in pts.windows(2) {
        renderer.draw_line_aa(w[0], w[1], Color::DARK_GRAY, model);
    }

    let style = StrokeStyle::solid_screen_px(thickness, color)
        .with_cap(LineCap::Butt)
        .with_join(join);
    renderer.stroke_polyline(&poly, &style, model);
}

fn draw_demo(renderer: &mut Renderer) {
    renderer.clear(Color::RGB(80, 80, 85));

    let thickness = 42.0;

    // Four rows of the same long polyline with different joins.
    draw_row(renderer, 80.0, LineJoin::Bevel, Color::CYAN, thickness);
    draw_row(
        renderer,
        200.0,
        LineJoin::Miter { limit: 1000.0 },
        Color::MAGENTA,
        thickness,
    );
    draw_row(
        renderer,
        320.0,
        LineJoin::Miter { limit: 1.0 },
        Color::PURPLE,
        thickness,
    );
    draw_row(renderer, 440.0, LineJoin::Round, Color::YELLOW, thickness);
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) = Window::new(
        "Lesson 4.3 - Line Joins (Bevel / Miter / Round)",
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
