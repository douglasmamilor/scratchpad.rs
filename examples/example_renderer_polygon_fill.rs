use scratchpad_rs::Color;
use scratchpad_rs::framebuffer::FrameBuffer;
use scratchpad_rs::math::{Point2, Vec2};
use scratchpad_rs::renderer::{FillRule, Renderer};
use scratchpad_rs::window::Window;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use std::time::Duration;

const INITIAL_WINDOW_WIDTH: usize = 1280;
const INITIAL_WINDOW_HEIGHT: usize = 720;

fn build_concave_polygon(center: Vec2, radius: f32) -> Vec<Point2> {
    // Simple "house" shape: square with a roof
    let cx = center.x;
    let cy = center.y;
    let r = radius;

    vec![
        Point2::new(cx - r, cy + r), // bottom left
        Point2::new(cx + r, cy + r), // bottom right
        Point2::new(cx + r, cy),     // mid right
        Point2::new(cx, cy - r),     // roof peak
        Point2::new(cx - r, cy),     // mid left
    ]
}

fn build_self_intersecting_polygon(center: Vec2, radius: f32) -> Vec<Point2> {
    // Simple 5-point star (self-intersecting)
    let cx = center.x;
    let cy = center.y;
    let r_outer = radius;
    let r_inner = radius * 0.4;
    let mut vertices = Vec::with_capacity(10);

    for i in 0..5 {
        let angle_outer = (i as f32) * std::f32::consts::TAU / 5.0 - std::f32::consts::FRAC_PI_2;
        let angle_inner =
            (i as f32 + 0.5) * std::f32::consts::TAU / 5.0 - std::f32::consts::FRAC_PI_2;

        vertices.push(Point2::new(
            cx + r_outer * angle_outer.cos(),
            cy + r_outer * angle_outer.sin(),
        ));
        vertices.push(Point2::new(
            cx + r_inner * angle_inner.cos(),
            cy + r_inner * angle_inner.sin(),
        ));
    }

    vertices
}

fn draw_polygon_demos(renderer: &mut Renderer) {
    renderer.clear(Color::DARK_GRAY);

    // Left: concave polygon filled with EvenOdd rule
    let concave_even_odd = build_concave_polygon(Vec2::new(320.0, 240.0), 80.0);
    renderer.fill_polygon(concave_even_odd, Color::CYAN, FillRule::EvenOdd);

    // Center: concave polygon filled with NonZeroWinding rule
    let concave_non_zero = build_concave_polygon(Vec2::new(640.0, 240.0), 80.0);
    renderer.fill_polygon(concave_non_zero, Color::MAGENTA, FillRule::NonZeroWinding);

    // Right: self-intersecting star with EvenOdd rule
    let star_even_odd = build_self_intersecting_polygon(Vec2::new(960.0, 240.0), 100.0);
    renderer.fill_polygon(star_even_odd, Color::YELLOW, FillRule::EvenOdd);

    // Overlay outlines so edges are clear
    let outline_color = Color::WHITE;
    let concave_outline = build_concave_polygon(Vec2::new(320.0, 240.0), 80.0)
        .into_iter()
        .map(|p| Vec2::new(p.x, p.y))
        .collect::<Vec<_>>();
    renderer.draw_polygon(
        &concave_outline,
        outline_color,
        scratchpad_rs::math::Mat3::IDENTITY,
    );

    let concave_outline_nz = build_concave_polygon(Vec2::new(640.0, 240.0), 80.0)
        .into_iter()
        .map(|p| Vec2::new(p.x, p.y))
        .collect::<Vec<_>>();
    renderer.draw_polygon(
        &concave_outline_nz,
        outline_color,
        scratchpad_rs::math::Mat3::IDENTITY,
    );

    let star_outline = build_self_intersecting_polygon(Vec2::new(960.0, 240.0), 100.0)
        .into_iter()
        .map(|p| Vec2::new(p.x, p.y))
        .collect::<Vec<_>>();
    renderer.draw_polygon(
        &star_outline,
        outline_color,
        scratchpad_rs::math::Mat3::IDENTITY,
    );
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) = Window::new(
        "Polygon Fill Demo (EvenOdd vs NonZero)",
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
            draw_polygon_demos(&mut renderer);
        }

        window.present(&framebuffer)?;

        let frame_time = frame_start.elapsed();
        let target = Duration::from_nanos(1_000_000_000 / 60);
        if let Some(remaining) = target.checked_sub(frame_time) {
            std::thread::sleep(remaining);
        }
    }
}
