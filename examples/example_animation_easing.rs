use scratchpad_rs::Color;
use scratchpad_rs::animation::Animation;
use scratchpad_rs::animation::{
    ease_in_back, ease_in_cubic, ease_in_out_back, ease_in_out_cubic, ease_in_out_quad,
    ease_in_out_sine, ease_in_quad, ease_in_sine, ease_out_back, ease_out_bounce, ease_out_cubic,
    ease_out_elastic, ease_out_quad, ease_out_sine, linear,
};
use scratchpad_rs::framebuffer::FrameBuffer;
use scratchpad_rs::math::{Mat3, Vec2};
use scratchpad_rs::renderer::Renderer;
use scratchpad_rs::window::Window;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use std::time::Duration;

const INITIAL_WINDOW_WIDTH: usize = 1280;
const INITIAL_WINDOW_HEIGHT: usize = 720;

struct EasingDemo {
    _name: &'static str,
    animation: Animation<Vec2>,
    _easing_fn: fn(f32) -> f32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) = Window::new(
        "lesson_3_2 - Animation & Easing Functions",
        INITIAL_WINDOW_WIDTH as u32,
        INITIAL_WINDOW_HEIGHT as u32,
    )?;

    let mut frame_buffer = FrameBuffer::new(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT);

    // Create multiple animations with different easing functions
    let start_x = 100.0;
    let end_x = 1100.0;
    let center_y = (INITIAL_WINDOW_HEIGHT as f32) / 2.0;
    let spacing = 50.0;
    let duration = 2.0;

    let mut demos = vec![
        EasingDemo {
            _name: "Linear",
            animation: Animation::with_easing(
                Vec2::new(start_x, center_y - spacing * 6.0),
                Vec2::new(end_x, center_y - spacing * 6.0),
                duration,
                linear,
            ),
            _easing_fn: linear,
        },
        EasingDemo {
            _name: "Ease In Quad",
            animation: Animation::with_easing(
                Vec2::new(start_x, center_y - spacing * 5.0),
                Vec2::new(end_x, center_y - spacing * 5.0),
                duration,
                ease_in_quad,
            ),
            _easing_fn: ease_in_quad,
        },
        EasingDemo {
            _name: "Ease Out Quad",
            animation: Animation::with_easing(
                Vec2::new(start_x, center_y - spacing * 4.0),
                Vec2::new(end_x, center_y - spacing * 4.0),
                duration,
                ease_out_quad,
            ),
            _easing_fn: ease_out_quad,
        },
        EasingDemo {
            _name: "Ease In Out Quad",
            animation: Animation::with_easing(
                Vec2::new(start_x, center_y - spacing * 3.0),
                Vec2::new(end_x, center_y - spacing * 3.0),
                duration,
                ease_in_out_quad,
            ),
            _easing_fn: ease_in_out_quad,
        },
        EasingDemo {
            _name: "Ease In Cubic",
            animation: Animation::with_easing(
                Vec2::new(start_x, center_y - spacing * 2.0),
                Vec2::new(end_x, center_y - spacing * 2.0),
                duration,
                ease_in_cubic,
            ),
            _easing_fn: ease_in_cubic,
        },
        EasingDemo {
            _name: "Ease Out Cubic",
            animation: Animation::with_easing(
                Vec2::new(start_x, center_y - spacing * 1.0),
                Vec2::new(end_x, center_y - spacing * 1.0),
                duration,
                ease_out_cubic,
            ),
            _easing_fn: ease_out_cubic,
        },
        EasingDemo {
            _name: "Ease In Out Cubic",
            animation: Animation::with_easing(
                Vec2::new(start_x, center_y),
                Vec2::new(end_x, center_y),
                duration,
                ease_in_out_cubic,
            ),
            _easing_fn: ease_in_out_cubic,
        },
        EasingDemo {
            _name: "Ease In Sine",
            animation: Animation::with_easing(
                Vec2::new(start_x, center_y + spacing * 1.0),
                Vec2::new(end_x, center_y + spacing * 1.0),
                duration,
                ease_in_sine,
            ),
            _easing_fn: ease_in_sine,
        },
        EasingDemo {
            _name: "Ease Out Sine",
            animation: Animation::with_easing(
                Vec2::new(start_x, center_y + spacing * 2.0),
                Vec2::new(end_x, center_y + spacing * 2.0),
                duration,
                ease_out_sine,
            ),
            _easing_fn: ease_out_sine,
        },
        EasingDemo {
            _name: "Ease In Out Sine",
            animation: Animation::with_easing(
                Vec2::new(start_x, center_y + spacing * 3.0),
                Vec2::new(end_x, center_y + spacing * 3.0),
                duration,
                ease_in_out_sine,
            ),
            _easing_fn: ease_in_out_sine,
        },
        EasingDemo {
            _name: "Ease In Back",
            animation: Animation::with_easing(
                Vec2::new(start_x, center_y + spacing * 4.0),
                Vec2::new(end_x, center_y + spacing * 4.0),
                duration,
                ease_in_back,
            ),
            _easing_fn: ease_in_back,
        },
        EasingDemo {
            _name: "Ease Out Back",
            animation: Animation::with_easing(
                Vec2::new(start_x, center_y + spacing * 5.0),
                Vec2::new(end_x, center_y + spacing * 5.0),
                duration,
                ease_out_back,
            ),
            _easing_fn: ease_out_back,
        },
        EasingDemo {
            _name: "Ease In Out Back",
            animation: Animation::with_easing(
                Vec2::new(start_x, center_y + spacing * 6.0),
                Vec2::new(end_x, center_y + spacing * 6.0),
                duration,
                ease_in_out_back,
            ),
            _easing_fn: ease_in_out_back,
        },
        EasingDemo {
            _name: "Ease Out Bounce",
            animation: Animation::with_easing(
                Vec2::new(start_x, center_y + spacing * 7.0),
                Vec2::new(end_x, center_y + spacing * 7.0),
                duration,
                ease_out_bounce,
            ),
            _easing_fn: ease_out_bounce,
        },
        EasingDemo {
            _name: "Ease Out Elastic",
            animation: Animation::with_easing(
                Vec2::new(start_x, center_y + spacing * 8.0),
                Vec2::new(end_x, center_y + spacing * 8.0),
                duration,
                ease_out_elastic,
            ),
            _easing_fn: ease_out_elastic,
        },
    ];

    let mut paused = false;
    let mut show_labels = true;

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
                }

                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    match key {
                        Keycode::Space => {
                            paused = !paused;
                        }
                        Keycode::R => {
                            // Reset all animations
                            for demo in &mut demos {
                                demo.animation.reset();
                            }
                        }
                        Keycode::L => {
                            show_labels = !show_labels;
                        }
                        _ => {}
                    }
                }

                _ => {}
            }
        }

        // Update animations
        if !paused {
            let delta_time = 1.0 / 60.0; // Assume 60 FPS
            for demo in &mut demos {
                demo.animation.update(delta_time);

                // Loop animations
                if demo.animation.is_complete() {
                    demo.animation.reset();
                }
            }
        }

        // Clear framebuffer
        frame_buffer.clear(Color::BLACK.to_u32());

        let mut renderer = Renderer::new(&mut frame_buffer);

        // Draw start and end markers
        let gray = Color::GRAY;
        for y_offset in [-spacing * 6.0, spacing * 8.0] {
            let y = center_y + y_offset;
            renderer.draw_line_aa(
                Vec2::new(start_x, y),
                Vec2::new(start_x + 20.0, y),
                gray,
                Mat3::IDENTITY,
            );
            renderer.draw_line_aa(
                Vec2::new(end_x - 20.0, y),
                Vec2::new(end_x, y),
                gray,
                Mat3::IDENTITY,
            );
        }

        // Draw each animated object
        for demo in &demos {
            let pos = demo.animation.value();
            let progress = demo.animation.progress();

            // Color based on progress (red -> green -> blue)
            let r = if progress < 0.5 {
                1.0 - progress * 2.0
            } else {
                0.0
            } as u8;
            let g = if progress < 0.5 {
                progress * 2.0
            } else {
                1.0 - (progress - 0.5) * 2.0
            } as u8;
            let b = if progress > 0.5 {
                (progress - 0.5) * 2.0
            } else {
                0.0
            } as u8;

            let color = Color::RGB(r, g, b);

            // Draw circle at current position
            renderer.fill_circle(pos, 8.0, color, Mat3::IDENTITY);

            // Draw label placeholder
            if show_labels {
                let label_x = pos.x - 60.0;
                let label_y = pos.y - 20.0;
                renderer.fill_rect(
                    Vec2::new(label_x, label_y),
                    Vec2::new(label_x + 120.0, label_y + 12.0),
                    Color::WHITE,
                    Mat3::IDENTITY,
                );
            }
        }

        // Draw info placeholder
        let info_y = 20.0;
        renderer.fill_rect(
            Vec2::new(10.0, info_y),
            Vec2::new(410.0, info_y + 80.0),
            Color::WHITE,
            Mat3::IDENTITY,
        );

        // Present frame
        window.present(&frame_buffer)?;

        // Frame rate limiting
        let frame_time = frame_start.elapsed();
        let target_frame_time = Duration::from_secs_f32(1.0 / 60.0);
        if frame_time < target_frame_time {
            std::thread::sleep(target_frame_time - frame_time);
        }
    }
}
