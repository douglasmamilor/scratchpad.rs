use scratchpad_rs::framebuffer::FrameBuffer;
use scratchpad_rs::renderer::Renderer;
use scratchpad_rs::window::Window;
use scratchpad_rs::{Color, Mat3, Vec2};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::{Keycode, Scancode};
use std::error::Error;
use std::thread;
use std::time::{Duration, Instant};

const INITIAL_WIDTH: usize = 800;
const INITIAL_HEIGHT: usize = 600;
const PLAYER_SIZE: f32 = 40.0;
const PLAYER_SPEED: f32 = 240.0;
const MAX_DELTA_TIME: f32 = 0.1;
const TARGET_FRAME_TIME: Duration = Duration::from_nanos(16_666_667);

#[derive(Debug, Default)]
struct InputState {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}

#[derive(Debug)]
struct AppState {
    position: Vec2,
    size: Vec2,
    speed: f32,
}

impl AppState {
    fn new(framebuffer_size: Vec2) -> Self {
        let size = Vec2::new(PLAYER_SIZE, PLAYER_SIZE);

        Self {
            position: (framebuffer_size - size) * 0.5,
            size,
            speed: PLAYER_SPEED,
        }
    }

    fn update(&mut self, input: &InputState, delta_time: f32, bounds: Vec2) {
        let mut direction = Vec2::ZERO;

        if input.left {
            direction.x -= 1.0;
        }
        if input.right {
            direction.x += 1.0;
        }
        if input.up {
            direction.y -= 1.0;
        }
        if input.down {
            direction.y += 1.0;
        }

        direction = direction.normalize_or_zero();
        self.position += direction * self.speed * delta_time;

        let max_x = (bounds.x - self.size.x).max(0.0);
        let max_y = (bounds.y - self.size.y).max(0.0);

        self.position.x = self.position.x.clamp(0.0, max_x);
        self.position.y = self.position.y.clamp(0.0, max_y);
    }

    fn render(&self, renderer: &mut Renderer<'_>) {
        renderer.fill_rect(
            self.position,
            self.position + self.size,
            Color::CYAN,
            Mat3::IDENTITY,
        );
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let (mut window, mut event_pump) = Window::new_resizable(
        "Canonical Application Loop",
        INITIAL_WIDTH as u32,
        INITIAL_HEIGHT as u32,
    )?;

    let mut framebuffer = FrameBuffer::new(INITIAL_WIDTH, INITIAL_HEIGHT);
    let initial_bounds = Vec2::new(INITIAL_WIDTH as f32, INITIAL_HEIGHT as f32);
    let mut app = AppState::new(initial_bounds);

    let mut last_frame = Instant::now();

    'running: loop {
        let frame_start = Instant::now();
        let delta_time = (frame_start - last_frame).as_secs_f32().min(MAX_DELTA_TIME);
        last_frame = frame_start;

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
                } => break 'running,

                Event::Window {
                    win_event: WindowEvent::Resized(width, height),
                    ..
                }
                | Event::Window {
                    win_event: WindowEvent::SizeChanged(width, height),
                    ..
                } if width > 0 && height > 0 => {
                    framebuffer.resize(width as usize, height as usize);
                }

                _ => {}
            }
        }

        let keyboard = event_pump.keyboard_state();
        let input = InputState {
            left: keyboard.is_scancode_pressed(Scancode::Left)
                || keyboard.is_scancode_pressed(Scancode::A),
            right: keyboard.is_scancode_pressed(Scancode::Right)
                || keyboard.is_scancode_pressed(Scancode::D),
            up: keyboard.is_scancode_pressed(Scancode::Up)
                || keyboard.is_scancode_pressed(Scancode::W),
            down: keyboard.is_scancode_pressed(Scancode::Down)
                || keyboard.is_scancode_pressed(Scancode::S),
        };

        let bounds = Vec2::new(framebuffer.width() as f32, framebuffer.height() as f32);
        app.update(&input, delta_time, bounds);

        {
            let mut renderer = Renderer::new(&mut framebuffer);
            renderer.clear(Color::DARK_GRAY);
            app.render(&mut renderer);
        }

        window.present(&framebuffer)?;

        let remaining = TARGET_FRAME_TIME.saturating_sub(frame_start.elapsed());
        thread::sleep(remaining);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 0.001;

    fn assert_near(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < EPSILON,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn movement_uses_units_per_second() {
        let bounds = Vec2::new(800.0, 600.0);
        let mut app = AppState::new(bounds);
        let start = app.position;

        let input = InputState {
            right: true,
            ..InputState::default()
        };

        app.update(&input, 0.5, bounds);

        assert_near(app.position.x, start.x + 120.0);
        assert_near(app.position.y, start.y);
    }

    #[test]
    fn diagonal_movement_is_normalized() {
        let bounds = Vec2::new(800.0, 600.0);
        let mut app = AppState::new(bounds);
        let start = app.position;

        let input = InputState {
            right: true,
            down: true,
            ..InputState::default()
        };

        app.update(&input, 0.5, bounds);

        let displacement = app.position - start;

        // Total distance should still be speed × time:
        // 240 pixels/second × 0.5 seconds = 120 pixels.
        assert_near(displacement.len(), 120.0);

        // A 45-degree diagonal should move equally along both axes.
        assert_near(displacement.x, displacement.y);
    }

    #[test]
    fn movement_is_clamped_to_framebuffer_bounds() {
        let bounds = Vec2::new(800.0, 600.0);
        let mut app = AppState::new(bounds);

        app.position = Vec2::new(0.0, 0.0);

        let up_left = InputState {
            left: true,
            up: true,
            ..InputState::default()
        };

        app.update(&up_left, 1.0, bounds);

        assert_eq!(app.position, Vec2::new(0.0, 0.0));

        app.position = Vec2::new(800.0, 600.0);

        let down_right = InputState {
            right: true,
            down: true,
            ..InputState::default()
        };

        app.update(&down_right, 1.0, bounds);

        assert_eq!(app.position, Vec2::new(760.0, 560.0));
    }
}
