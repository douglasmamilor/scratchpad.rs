use scratchpad_rs::Color;
use scratchpad_rs::animation::KeyFrameAnimation;
use scratchpad_rs::animation::Keyframe;
use scratchpad_rs::animation::{ease_in_out_cubic, ease_in_quad, ease_out_bounce, ease_out_quad};
use scratchpad_rs::framebuffer::FrameBuffer;
use scratchpad_rs::math::{Mat3, Vec2};
use scratchpad_rs::renderer::Renderer;
use scratchpad_rs::window::Window;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use std::time::Duration;

const INITIAL_WINDOW_WIDTH: usize = 1280;
const INITIAL_WINDOW_HEIGHT: usize = 720;

struct Demo {
    _name: &'static str,
    anim: KeyFrameAnimation<Vec2>,
    color: Color,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) = Window::new(
        "lesson_3_2 - Keyframe Animation",
        INITIAL_WINDOW_WIDTH as u32,
        INITIAL_WINDOW_HEIGHT as u32,
    )?;

    let mut frame_buffer = FrameBuffer::new(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT);

    let center_x = INITIAL_WINDOW_WIDTH as f32 / 2.0;
    let center_y = INITIAL_WINDOW_HEIGHT as f32 / 2.0;
    let radius = 200.0;

    // Demo 1: Simple linear keyframe animation (triangle path)
    let mut anim1 = KeyFrameAnimation::new();
    anim1.add_keyframe(0.0, Vec2::new(center_x, center_y - radius));
    anim1.add_keyframe(
        1.0,
        Vec2::new(center_x + radius * 0.866, center_y + radius * 0.5),
    );
    anim1.add_keyframe(
        2.0,
        Vec2::new(center_x - radius * 0.866, center_y + radius * 0.5),
    );
    anim1.add_keyframe(3.0, Vec2::new(center_x, center_y - radius)); // Back to start
    anim1.set_looping(true);
    anim1.play();

    // Demo 2: Keyframe animation with per-segment easing
    let mut anim2 = KeyFrameAnimation::new();
    let mut kf0 = Keyframe::new(0.0, Vec2::new(100.0, center_y));
    kf0.set_easing_out(ease_in_quad); // Slow start
    anim2.add_keyframes(vec![kf0]);
    let mut kf1 = Keyframe::new(1.0, Vec2::new(center_x, center_y));
    kf1.set_easing_out(ease_out_quad); // Fast start, slow end
    anim2.add_keyframes(vec![kf1]);
    anim2.add_keyframe(2.0, Vec2::new(1180.0, center_y));
    anim2.set_looping(true);
    anim2.play();

    // Demo 3: Bouncing ball with ease-out-bounce
    let mut anim3 = KeyFrameAnimation::new();
    let mut kf_bounce_start = Keyframe::new(0.0, Vec2::new(center_x, 100.0));
    kf_bounce_start.set_easing_out(ease_out_bounce);
    anim3.add_keyframes(vec![kf_bounce_start]);
    anim3.add_keyframe(1.5, Vec2::new(center_x, 620.0));
    anim3.set_looping(true);
    anim3.play();

    // Demo 4: Complex path with multiple easing functions
    let mut anim4 = KeyFrameAnimation::new();
    let mut kf4_0 = Keyframe::new(0.0, Vec2::new(center_x, 100.0));
    kf4_0.set_easing_out(ease_in_out_cubic);
    anim4.add_keyframes(vec![kf4_0]);
    anim4.add_keyframe(1.0, Vec2::new(1180.0, center_y));
    let mut kf4_1 = Keyframe::new(2.0, Vec2::new(center_x, 620.0));
    kf4_1.set_easing_out(ease_in_out_cubic);
    anim4.add_keyframes(vec![kf4_1]);
    anim4.add_keyframe(3.0, Vec2::new(100.0, center_y));
    anim4.add_keyframe(4.0, Vec2::new(center_x, 100.0)); // Back to start
    anim4.set_looping(true);
    anim4.play();

    let mut demos = vec![
        Demo {
            _name: "Triangle Path (Linear)",
            anim: anim1,
            color: Color::RED,
        },
        Demo {
            _name: "Horizontal (Mixed Easing)",
            anim: anim2,
            color: Color::GREEN,
        },
        Demo {
            _name: "Bouncing Ball",
            anim: anim3,
            color: Color::BLUE,
        },
        Demo {
            _name: "Complex Path",
            anim: anim4,
            color: Color::YELLOW,
        },
    ];

    let mut paused = false;
    let mut show_trails = true;
    let mut show_keyframes = true;

    // Trail storage for each demo
    let mut trails: Vec<Vec<Vec2>> = vec![Vec::new(); demos.len()];
    const MAX_TRAIL_LENGTH: usize = 100;

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
                            if paused {
                                for demo in &mut demos {
                                    demo.anim.pause();
                                }
                            } else {
                                for demo in &mut demos {
                                    demo.anim.play();
                                }
                            }
                        }
                        Keycode::R => {
                            // Reset all animations
                            for demo in &mut demos {
                                demo.anim.stop();
                                demo.anim.play();
                            }
                            trails = vec![Vec::new(); demos.len()];
                        }
                        Keycode::T => {
                            show_trails = !show_trails;
                        }
                        Keycode::K => {
                            show_keyframes = !show_keyframes;
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
            for (i, demo) in demos.iter_mut().enumerate() {
                demo.anim.update(delta_time);

                // Update trail
                if show_trails {
                    let pos = demo.anim.value();
                    trails[i].push(pos);
                    if trails[i].len() > MAX_TRAIL_LENGTH {
                        trails[i].remove(0);
                    }
                }
            }
        }

        // Clear framebuffer
        frame_buffer.clear(Color::BLACK.to_u32());

        let mut renderer = Renderer::new(&mut frame_buffer);

        // Draw keyframes for each demo
        if show_keyframes {
            // Note: We can't access keyframes directly from the animation,
            // so we skip drawing keyframe positions here
            // In a real implementation, you'd need a method to get keyframe positions
        }

        // Draw trails
        if show_trails {
            for (i, trail) in trails.iter().enumerate() {
                if trail.len() < 2 {
                    continue;
                }

                let demo = &demos[i];
                let trail_color = Color::RGB(demo.color.r / 3, demo.color.g / 3, demo.color.b / 3);

                for j in 0..trail.len() - 1 {
                    renderer.draw_line_aa(trail[j], trail[j + 1], trail_color, Mat3::IDENTITY);
                }
            }
        }

        // Draw animated objects
        for demo in &demos {
            let pos = demo.anim.value();
            renderer.fill_circle(pos, 12.0, demo.color, Mat3::IDENTITY);
        }

        // Draw info panel placeholder
        let info_y = 20.0;
        renderer.fill_rect(
            Vec2::new(10.0, info_y),
            Vec2::new(410.0, info_y + 100.0),
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
