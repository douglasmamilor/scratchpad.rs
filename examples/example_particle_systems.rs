use scratchpad_rs::framebuffer::FrameBuffer;
use scratchpad_rs::math::Vec2;
use scratchpad_rs::particle::Emitter;
use scratchpad_rs::renderer::Renderer;
use scratchpad_rs::window::Window;
use scratchpad_rs::Color;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use std::time::{Duration, Instant};

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DemoMode {
    Fire,
    Smoke,
    Rain,
    Sparkles,
    Combined,
}

impl DemoMode {
    fn next(self) -> Self {
        match self {
            Self::Fire => Self::Smoke,
            Self::Smoke => Self::Rain,
            Self::Rain => Self::Sparkles,
            Self::Sparkles => Self::Combined,
            Self::Combined => Self::Fire,
        }
    }

    fn prev(self) -> Self {
        match self {
            Self::Fire => Self::Combined,
            Self::Smoke => Self::Fire,
            Self::Rain => Self::Smoke,
            Self::Sparkles => Self::Rain,
            Self::Combined => Self::Sparkles,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Fire => "Fire",
            Self::Smoke => "Smoke",
            Self::Rain => "Rain",
            Self::Sparkles => "Sparkles (move mouse)",
            Self::Combined => "Fire + Smoke Combined",
        }
    }
}

fn create_emitters_for_mode(mode: DemoMode) -> Vec<Emitter> {
    match mode {
        DemoMode::Fire => {
            vec![Emitter::fire(Vec2::new(WIDTH as f32 / 2.0, HEIGHT as f32 - 100.0))]
        }
        DemoMode::Smoke => {
            vec![Emitter::smoke(Vec2::new(WIDTH as f32 / 2.0, HEIGHT as f32 - 100.0))]
        }
        DemoMode::Rain => {
            vec![Emitter::rain(
                Vec2::new(0.0, -20.0),
                Vec2::new(WIDTH as f32, -20.0),
            )]
        }
        DemoMode::Sparkles => {
            vec![Emitter::sparkle_trail(Vec2::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0))]
        }
        DemoMode::Combined => {
            let base_x = WIDTH as f32 / 2.0;
            let base_y = HEIGHT as f32 - 100.0;
            vec![
                Emitter::fire(Vec2::new(base_x, base_y)),
                Emitter::smoke(Vec2::new(base_x, base_y - 60.0)),
            ]
        }
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) =
        Window::new("Lesson 7.2 - Particle Systems", WIDTH as u32, HEIGHT as u32)?;

    let mut framebuffer = FrameBuffer::new(WIDTH, HEIGHT);

    let mut mode = DemoMode::Fire;
    let mut emitters = create_emitters_for_mode(mode);
    let mut sparks_emitter = Emitter::sparks_burst(Vec2::new(0.0, 0.0));

    let mut last_frame = Instant::now();

    println!("Particle Systems Demo");
    println!("  Left/Right: Change effect");
    println!("  Space: Trigger spark burst at mouse position");
    println!("  Esc: Quit");
    println!();
    println!("Current: {}", mode.name());

    'running: loop {
        let now = Instant::now();
        let delta_time = (now - last_frame).as_secs_f32();
        last_frame = now;

        let mut trigger_sparks = false;

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

                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    mode = mode.next();
                    emitters = create_emitters_for_mode(mode);
                    println!("Effect: {}", mode.name());
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    mode = mode.prev();
                    emitters = create_emitters_for_mode(mode);
                    println!("Effect: {}", mode.name());
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    trigger_sparks = true;
                }

                Event::Window {
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => framebuffer.resize(w as usize, h as usize),

                _ => {}
            }
        }

        // Get mouse state after event loop (avoids borrow conflict)
        let mouse_state = event_pump.mouse_state();
        let mouse_pos = Vec2::new(mouse_state.x() as f32, mouse_state.y() as f32);

        if trigger_sparks {
            sparks_emitter.position = mouse_pos;
            sparks_emitter.emit_burst(40);
            println!("Spark burst at ({}, {})", mouse_state.x(), mouse_state.y());
        }

        // Update sparkle emitter position to follow mouse (in Sparkles mode)
        if mode == DemoMode::Sparkles {
            if let Some(emitter) = emitters.first_mut() {
                emitter.position = mouse_pos;
            }
        }

        // Update all emitters
        for emitter in &mut emitters {
            emitter.update(delta_time);
        }
        sparks_emitter.update(delta_time);

        // Render
        {
            let mut renderer = Renderer::new(&mut framebuffer);

            // Dark background for better particle visibility
            renderer.clear(Color::RGB(20, 20, 30));

            // Draw main emitters
            for emitter in &emitters {
                renderer.draw_particles(emitter);
            }

            // Draw spark bursts
            renderer.draw_particles(&sparks_emitter);
        }

        window.present(&framebuffer)?;

        // Frame rate limiting (60 FPS)
        let frame_time = last_frame.elapsed();
        let target_frame_time = Duration::from_secs_f32(1.0 / 60.0);
        if frame_time < target_frame_time {
            std::thread::sleep(target_frame_time - frame_time);
        }
    }
}
