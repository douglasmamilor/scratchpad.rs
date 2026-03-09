use scratchpad_rs::Color;
use scratchpad_rs::framebuffer::FrameBuffer;
use scratchpad_rs::math::{Mat3, Point2, Vec2, mod_pos};
use scratchpad_rs::renderer::{
    LineCap, PatternSpace, PolyLine, Renderer, StrokePattern, StrokeSpace, StrokeStyle,
};
use scratchpad_rs::window::Window;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use std::time::Duration;

const INITIAL_WINDOW_WIDTH: usize = 1280;
const INITIAL_WINDOW_HEIGHT: usize = 720;

fn draw_dotted(
    renderer: &mut Renderer,
    base: &PolyLine,
    dot_space: f32,
    dot_radius: f32,
    phase: f32,
    color: Color,
) {
    let total = base.len();
    if total <= 0.0 {
        return;
    }

    let step = dot_space.max(1e-6);
    let start = mod_pos(phase, step);

    let mut s = start;
    while s <= total {
        let center = base.point_at_len(s);
        // Render dots with the dedicated circle rasterizer to avoid polygon-fill artifacts.
        renderer.fill_circle(center, dot_radius, color, Mat3::IDENTITY);
        s += step;
    }
}

fn draw_dashed(
    renderer: &mut Renderer,
    base: &PolyLine,
    dash_len: f32,
    gap_len: f32,
    phase: f32,
    thickness_px: f32,
    color: Color,
) {
    let style = StrokeStyle::solid_screen_px(thickness_px, color)
        .with_cap(LineCap::Butt)
        .with_space(StrokeSpace::ScreenSpace {
            thickness: thickness_px.max(0.0) as u64,
        })
        .with_pattern(StrokePattern::Dashed {
            dash_length: dash_len,
            gap_length: gap_len,
            phase,
            enabled: true,
            space: PatternSpace::StrokeSpace,
        });

    renderer.stroke_polyline_patterned(base, &style, Mat3::IDENTITY);
}

fn build_demo_paths() -> (PolyLine, PolyLine) {
    let dashed_path = PolyLine::new(
        vec![Point2::new(80.0, 140.0), Point2::new(1200.0, 140.0)],
        false,
    );

    // A multi-segment polyline (shows that we do NOT handle joins/caps yet).
    let dotted_path = PolyLine::new(
        vec![
            Point2::new(120.0, 420.0),
            Point2::new(320.0, 320.0),
            Point2::new(520.0, 460.0),
            Point2::new(720.0, 340.0),
            Point2::new(920.0, 480.0),
            Point2::new(1120.0, 380.0),
        ],
        false,
    );

    (dashed_path, dotted_path)
}

fn draw_demo(renderer: &mut Renderer, t: f32) {
    renderer.clear(Color::DARK_GRAY);

    let (dashed_path, dotted_path) = build_demo_paths();

    // Solid thick line (no pattern)
    {
        let style = StrokeStyle::solid_screen_px(14.0, Color::MAGENTA).with_cap(LineCap::Butt);
        renderer.stroke_segment(
            Vec2::new(80.0, 240.0),
            Vec2::new(1200.0, 240.0),
            &style,
            Mat3::IDENTITY,
        );
    }

    // Reference (thin AA line)
    renderer.draw_line_aa(
        Vec2::new(80.0, 140.0),
        Vec2::new(1200.0, 140.0),
        Color::GRAY,
        Mat3::IDENTITY,
    );

    let dash_phase = t * 120.0;
    let dot_phase = t * 60.0;

    draw_dashed(
        renderer,
        &dashed_path,
        48.0,
        24.0,
        dash_phase,
        14.0,
        Color::CYAN,
    );

    // Reference (thin polyline)
    for w in dotted_path.points().windows(2) {
        renderer.draw_line_aa(w[0], w[1], Color::GRAY, Mat3::IDENTITY);
    }

    draw_dotted(renderer, &dotted_path, 42.0, 8.0, dot_phase, Color::YELLOW);
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) = Window::new(
        "Lesson 4.3 - Advanced Lines (Thick + Dashed/Dotted)",
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

        let t = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as f32)
            * 0.001;

        {
            let mut renderer = Renderer::new(&mut framebuffer);
            draw_demo(&mut renderer, t);
        }

        window.present(&framebuffer)?;

        let frame_time = frame_start.elapsed();
        let target = Duration::from_nanos(1_000_000_000 / 60);
        if let Some(remaining) = target.checked_sub(frame_time) {
            std::thread::sleep(remaining);
        }
    }
}
