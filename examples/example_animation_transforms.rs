use scratchpad_rs::Color;
use scratchpad_rs::animation::KeyFrameAnimation;
use scratchpad_rs::animation::Keyframe;
use scratchpad_rs::animation::{ease_out_back, ease_out_bounce};
use scratchpad_rs::framebuffer::FrameBuffer;
use scratchpad_rs::math::{Mat3, Vec2};
use scratchpad_rs::renderer::Renderer;
use scratchpad_rs::window::Window;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use std::time::Duration;

const INITIAL_WINDOW_WIDTH: usize = 1280;
const INITIAL_WINDOW_HEIGHT: usize = 720;

struct AnimatedObject {
    position_anim: KeyFrameAnimation<Vec2>,
    rotation_anim: KeyFrameAnimation<f32>,
    scale_anim: KeyFrameAnimation<f32>,
    color: Color,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) = Window::new(
        "lesson_3_2 - Animated Transforms (Keyframes)",
        INITIAL_WINDOW_WIDTH as u32,
        INITIAL_WINDOW_HEIGHT as u32,
    )?;

    let mut frame_buffer = FrameBuffer::new(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT);

    let center = Vec2::new(
        INITIAL_WINDOW_WIDTH as f32 / 2.0,
        INITIAL_WINDOW_HEIGHT as f32 / 2.0,
    );

    // Object 1: Rotating square with scaling
    let mut pos_anim1 = KeyFrameAnimation::new();
    pos_anim1.add_keyframe(0.0, center);
    pos_anim1.set_looping(true);
    pos_anim1.play();

    let mut rot_anim1 = KeyFrameAnimation::new();
    rot_anim1.add_keyframe(0.0, 0.0);
    rot_anim1.add_keyframe(2.0, std::f32::consts::PI * 2.0);
    rot_anim1.set_looping(true);
    rot_anim1.play();

    let mut scale_anim1 = KeyFrameAnimation::new();
    let mut kf_scale1_0 = Keyframe::new(0.0, 1.0);
    kf_scale1_0.set_easing_out(ease_out_bounce);
    scale_anim1.add_keyframes(vec![kf_scale1_0]);
    scale_anim1.add_keyframe(1.0, 1.5);
    scale_anim1.add_keyframe(2.0, 1.0);
    scale_anim1.set_looping(true);
    scale_anim1.play();

    // Object 2: Orbiting with rotation
    let mut pos_anim2 = KeyFrameAnimation::new();
    let radius = 150.0;
    pos_anim2.add_keyframe(0.0, center + Vec2::new(radius, 0.0));
    pos_anim2.add_keyframe(2.0, center + Vec2::new(0.0, radius));
    pos_anim2.add_keyframe(4.0, center + Vec2::new(-radius, 0.0));
    pos_anim2.add_keyframe(6.0, center + Vec2::new(0.0, -radius));
    pos_anim2.add_keyframe(8.0, center + Vec2::new(radius, 0.0));
    pos_anim2.set_looping(true);
    pos_anim2.play();

    let mut rot_anim2 = KeyFrameAnimation::new();
    rot_anim2.add_keyframe(0.0, 0.0);
    rot_anim2.add_keyframe(8.0, std::f32::consts::PI * 4.0);
    rot_anim2.set_looping(true);
    rot_anim2.play();

    let mut scale_anim2 = KeyFrameAnimation::new();
    scale_anim2.add_keyframe(0.0, 0.8);
    scale_anim2.add_keyframe(4.0, 1.2);
    scale_anim2.add_keyframe(8.0, 0.8);
    scale_anim2.set_looping(true);
    scale_anim2.play();

    // Object 3: Bouncing with overshoot
    let mut pos_anim3 = KeyFrameAnimation::new();
    let mut kf_pos3_0 = Keyframe::new(0.0, Vec2::new(center.x, 100.0));
    kf_pos3_0.set_easing_out(ease_out_back);
    pos_anim3.add_keyframes(vec![kf_pos3_0]);
    pos_anim3.add_keyframe(1.0, Vec2::new(center.x, 620.0));
    pos_anim3.set_looping(true);
    pos_anim3.play();

    let mut rot_anim3 = KeyFrameAnimation::new();
    rot_anim3.add_keyframe(0.0, 0.0);
    rot_anim3.add_keyframe(0.5, std::f32::consts::PI);
    rot_anim3.add_keyframe(1.0, std::f32::consts::PI * 2.0);
    rot_anim3.set_looping(true);
    rot_anim3.play();

    let mut scale_anim3 = KeyFrameAnimation::new();
    scale_anim3.add_keyframe(0.0, 1.0);
    scale_anim3.add_keyframe(0.5, 1.3);
    scale_anim3.add_keyframe(1.0, 1.0);
    scale_anim3.set_looping(true);
    scale_anim3.play();

    let mut objects = vec![
        AnimatedObject {
            position_anim: pos_anim1,
            rotation_anim: rot_anim1,
            scale_anim: scale_anim1,
            color: Color::RED,
        },
        AnimatedObject {
            position_anim: pos_anim2,
            rotation_anim: rot_anim2,
            scale_anim: scale_anim2,
            color: Color::GREEN,
        },
        AnimatedObject {
            position_anim: pos_anim3,
            rotation_anim: rot_anim3,
            scale_anim: scale_anim3,
            color: Color::BLUE,
        },
    ];

    let mut paused = false;
    let mut show_info = true;

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
                                for obj in &mut objects {
                                    obj.position_anim.pause();
                                    obj.rotation_anim.pause();
                                    obj.scale_anim.pause();
                                }
                            } else {
                                for obj in &mut objects {
                                    obj.position_anim.play();
                                    obj.rotation_anim.play();
                                    obj.scale_anim.play();
                                }
                            }
                        }
                        Keycode::R => {
                            // Reset all animations
                            for obj in &mut objects {
                                obj.position_anim.stop();
                                obj.rotation_anim.stop();
                                obj.scale_anim.stop();
                                obj.position_anim.play();
                                obj.rotation_anim.play();
                                obj.scale_anim.play();
                            }
                        }
                        Keycode::I => {
                            show_info = !show_info;
                        }
                        _ => {}
                    }
                }

                _ => {}
            }
        }

        // Update animations
        if !paused {
            let delta_time = 1.0 / 60.0;
            for obj in &mut objects {
                obj.position_anim.update(delta_time);
                obj.rotation_anim.update(delta_time);
                obj.scale_anim.update(delta_time);
            }
        }

        // Clear framebuffer
        frame_buffer.clear(Color::BLACK.to_u32());

        let mut renderer = Renderer::new(&mut frame_buffer);

        // Draw each animated object
        for obj in &objects {
            let pos = obj.position_anim.value();
            let rot = obj.rotation_anim.value();
            let scale = obj.scale_anim.value();

            // Build transformation matrix
            let transform = Mat3::translate(pos.x, pos.y)
                * Mat3::rotate(rot)
                * Mat3::scale(scale * 30.0, scale * 30.0);

            // Draw a square (4 corners)
            let corners = vec![
                Vec2::new(-1.0, -1.0), // Top-left
                Vec2::new(1.0, -1.0),  // Top-right
                Vec2::new(1.0, 1.0),   // Bottom-right
                Vec2::new(-1.0, 1.0),  // Bottom-left
            ];

            for i in 0..corners.len() {
                let start = transform.transform_vec2(corners[i]);
                let end = transform.transform_vec2(corners[(i + 1) % corners.len()]);
                renderer.draw_line_aa(start, end, obj.color, Mat3::IDENTITY);
            }
        }

        // Draw info panel placeholder
        if show_info {
            renderer.fill_rect(
                Vec2::new(10.0, 20.0),
                Vec2::new(410.0, 120.0),
                Color::WHITE,
                Mat3::IDENTITY,
            );
        }

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
