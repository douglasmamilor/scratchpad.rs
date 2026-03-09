use scratchpad_rs::Color;
use scratchpad_rs::animation::KeyFrameAnimation;
use scratchpad_rs::animation::Keyframe;
use scratchpad_rs::animation::ease_in_out_cubic;
use scratchpad_rs::camera::Camera;
use scratchpad_rs::framebuffer::FrameBuffer;
use scratchpad_rs::math::{Mat3, Point2, Rect, Vec2};
use scratchpad_rs::renderer::Renderer;
use scratchpad_rs::window::Window;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use std::time::Duration;

const INITIAL_WINDOW_WIDTH: usize = 1280;
const INITIAL_WINDOW_HEIGHT: usize = 720;

struct CameraState {
    position: Point2,
    zoom: f32,
    rotation: f32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) = Window::new(
        "lesson_3_2 - Smooth Camera Transitions",
        INITIAL_WINDOW_WIDTH as u32,
        INITIAL_WINDOW_HEIGHT as u32,
    )?;

    let mut frame_buffer = FrameBuffer::new(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT);
    let viewport = Rect::new(
        0.0,
        0.0,
        INITIAL_WINDOW_WIDTH as f32,
        INITIAL_WINDOW_HEIGHT as f32,
    );
    let mut camera = Camera::default(viewport);

    // Define camera states
    let states = vec![
        CameraState {
            position: Point2::new(0.0, 0.0),
            zoom: 1.0,
            rotation: 0.0,
        },
        CameraState {
            position: Point2::new(200.0, 100.0),
            zoom: 1.5,
            rotation: 0.5,
        },
        CameraState {
            position: Point2::new(-150.0, -200.0),
            zoom: 0.8,
            rotation: -0.3,
        },
        CameraState {
            position: Point2::new(300.0, -100.0),
            zoom: 2.0,
            rotation: 1.0,
        },
    ];

    // Create keyframe animations for camera properties with easing
    let mut pos_x_anim = KeyFrameAnimation::new();
    let mut pos_y_anim = KeyFrameAnimation::new();
    let mut zoom_anim = KeyFrameAnimation::new();
    let mut rot_anim = KeyFrameAnimation::new();

    // Add keyframes for each state with easing
    for (i, state) in states.iter().enumerate() {
        let time = i as f32 * 3.0; // 3 seconds per transition

        // Create keyframes with easing (except the last one)
        if i < states.len() - 1 {
            let mut kf_x = Keyframe::new(time, state.position.x);
            kf_x.set_easing_out(ease_in_out_cubic);
            pos_x_anim.add_keyframes(vec![kf_x]);

            let mut kf_y = Keyframe::new(time, state.position.y);
            kf_y.set_easing_out(ease_in_out_cubic);
            pos_y_anim.add_keyframes(vec![kf_y]);

            let mut kf_z = Keyframe::new(time, state.zoom);
            kf_z.set_easing_out(ease_in_out_cubic);
            zoom_anim.add_keyframes(vec![kf_z]);

            let mut kf_r = Keyframe::new(time, state.rotation);
            kf_r.set_easing_out(ease_in_out_cubic);
            rot_anim.add_keyframes(vec![kf_r]);
        } else {
            // Last keyframe without easing
            pos_x_anim.add_keyframe(time, state.position.x);
            pos_y_anim.add_keyframe(time, state.position.y);
            zoom_anim.add_keyframe(time, state.zoom);
            rot_anim.add_keyframe(time, state.rotation);
        }
    }

    // Loop back to first state
    let final_time = (states.len() - 1) as f32 * 3.0;
    let mut kf_x_final = Keyframe::new(final_time, states[states.len() - 1].position.x);
    kf_x_final.set_easing_out(ease_in_out_cubic);
    pos_x_anim.add_keyframes(vec![kf_x_final]);

    let mut kf_y_final = Keyframe::new(final_time, states[states.len() - 1].position.y);
    kf_y_final.set_easing_out(ease_in_out_cubic);
    pos_y_anim.add_keyframes(vec![kf_y_final]);

    let mut kf_z_final = Keyframe::new(final_time, states[states.len() - 1].zoom);
    kf_z_final.set_easing_out(ease_in_out_cubic);
    zoom_anim.add_keyframes(vec![kf_z_final]);

    let mut kf_r_final = Keyframe::new(final_time, states[states.len() - 1].rotation);
    kf_r_final.set_easing_out(ease_in_out_cubic);
    rot_anim.add_keyframes(vec![kf_r_final]);

    pos_x_anim.add_keyframe(final_time + 3.0, states[0].position.x);
    pos_y_anim.add_keyframe(final_time + 3.0, states[0].position.y);
    zoom_anim.add_keyframe(final_time + 3.0, states[0].zoom);
    rot_anim.add_keyframe(final_time + 3.0, states[0].rotation);

    pos_x_anim.set_looping(true);
    pos_y_anim.set_looping(true);
    zoom_anim.set_looping(true);
    rot_anim.set_looping(true);

    pos_x_anim.play();
    pos_y_anim.play();
    zoom_anim.play();
    rot_anim.play();

    let mut paused = false;
    let mut show_grid = true;

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
                    let new_viewport = Rect::new(0.0, 0.0, w as f32, h as f32);
                    camera.set_viewport(new_viewport);
                }

                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    match key {
                        Keycode::Space => {
                            paused = !paused;
                            if paused {
                                pos_x_anim.pause();
                                pos_y_anim.pause();
                                zoom_anim.pause();
                                rot_anim.pause();
                            } else {
                                pos_x_anim.play();
                                pos_y_anim.play();
                                zoom_anim.play();
                                rot_anim.play();
                            }
                        }
                        Keycode::R => {
                            // Reset animations
                            pos_x_anim.stop();
                            pos_y_anim.stop();
                            zoom_anim.stop();
                            rot_anim.stop();
                            pos_x_anim.play();
                            pos_y_anim.play();
                            zoom_anim.play();
                            rot_anim.play();
                        }
                        Keycode::G => {
                            show_grid = !show_grid;
                        }
                        Keycode::Num1 | Keycode::Num2 | Keycode::Num3 | Keycode::Num4 => {
                            // Jump to specific state
                            let state_idx = match key {
                                Keycode::Num1 => 0,
                                Keycode::Num2 => 1,
                                Keycode::Num3 => 2,
                                Keycode::Num4 => 3,
                                _ => 0,
                            };
                            if state_idx < states.len() {
                                let time = state_idx as f32 * 3.0;
                                pos_x_anim.seek(time);
                                pos_y_anim.seek(time);
                                zoom_anim.seek(time);
                                rot_anim.seek(time);
                            }
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
            pos_x_anim.update(delta_time);
            pos_y_anim.update(delta_time);
            zoom_anim.update(delta_time);
            rot_anim.update(delta_time);
        }

        // Update camera from animations
        // We need to create a new camera with the animated values since Camera doesn't have setters
        let new_pos = Point2::new(pos_x_anim.value(), pos_y_anim.value());
        let new_zoom = zoom_anim.value();
        let new_rot = rot_anim.value();
        camera = Camera::new(new_pos, new_zoom, new_rot, camera.viewport());

        // Clear framebuffer
        frame_buffer.clear(Color::RGBA(15, 15, 25, 255).to_u32());

        let mut renderer = Renderer::new(&mut frame_buffer);
        let view_matrix = camera.view_matrix();

        // Draw world grid
        if show_grid {
            draw_world_grid(&mut renderer, &camera, &view_matrix);
        }

        // Draw world objects
        draw_world_objects(&mut renderer, &camera, &view_matrix);

        // Draw camera info
        draw_camera_info(
            &mut renderer,
            &camera,
            &pos_x_anim,
            &pos_y_anim,
            &zoom_anim,
            &rot_anim,
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

fn draw_world_grid(renderer: &mut Renderer, camera: &Camera, _view_matrix: &Mat3) {
    let grid_color = Color::RGBA(60, 60, 80, 255);
    let grid_major_color = Color::RGBA(100, 100, 120, 255);

    let viewport = camera.viewport();
    let top_left_world = camera.screen_to_world(Point2::new(0.0, 0.0));
    let bottom_right_world = camera.screen_to_world(Point2::new(viewport.width, viewport.height));

    let grid_spacing = 50.0;
    let major_grid_spacing = 200.0;

    // Draw vertical lines
    let start_x = (top_left_world.x / grid_spacing).floor() * grid_spacing;
    let end_x = (bottom_right_world.x / grid_spacing).ceil() * grid_spacing;

    for x in (start_x as i32)..=(end_x as i32) {
        let x_f32 = x as f32;
        let color = if (x_f32 % major_grid_spacing).abs() < 1.0 {
            grid_major_color
        } else if (x_f32 % grid_spacing).abs() < 1.0 {
            grid_color
        } else {
            continue;
        };

        let start_screen = camera.world_to_screen(Point2::new(x_f32, top_left_world.y));
        let end_screen = camera.world_to_screen(Point2::new(x_f32, bottom_right_world.y));
        renderer.draw_line_aa(start_screen, end_screen, color, Mat3::IDENTITY);
    }

    // Draw horizontal lines
    let start_y = (top_left_world.y / grid_spacing).floor() * grid_spacing;
    let end_y = (bottom_right_world.y / grid_spacing).ceil() * grid_spacing;

    for y in (start_y as i32)..=(end_y as i32) {
        let y_f32 = y as f32;
        let color = if (y_f32 % major_grid_spacing).abs() < 1.0 {
            grid_major_color
        } else if (y_f32 % grid_spacing).abs() < 1.0 {
            grid_color
        } else {
            continue;
        };

        let start_screen = camera.world_to_screen(Point2::new(top_left_world.x, y_f32));
        let end_screen = camera.world_to_screen(Point2::new(bottom_right_world.x, y_f32));
        renderer.draw_line_aa(start_screen, end_screen, color, Mat3::IDENTITY);
    }
}

fn draw_world_objects(renderer: &mut Renderer, camera: &Camera, _view_matrix: &Mat3) {
    // Draw some objects in world space
    let objects = vec![
        (Point2::new(0.0, 0.0), 30.0, Color::RED),
        (Point2::new(100.0, 100.0), 25.0, Color::GREEN),
        (Point2::new(-100.0, 150.0), 20.0, Color::BLUE),
        (Point2::new(200.0, -100.0), 35.0, Color::YELLOW),
        (Point2::new(-150.0, -150.0), 28.0, Color::CYAN),
    ];

    for (world_pos, radius, color) in objects {
        let screen_pos = camera.world_to_screen(world_pos);
        renderer.fill_circle(screen_pos, radius, color, Mat3::IDENTITY);
    }
}

fn draw_camera_info(
    renderer: &mut Renderer,
    _camera: &Camera,
    _pos_x_anim: &KeyFrameAnimation<f32>,
    _pos_y_anim: &KeyFrameAnimation<f32>,
    _zoom_anim: &KeyFrameAnimation<f32>,
    _rot_anim: &KeyFrameAnimation<f32>,
) {
    // Draw info panel placeholder
    renderer.fill_rect(
        Vec2::new(10.0, 20.0),
        Vec2::new(410.0, 180.0),
        Color::WHITE,
        Mat3::IDENTITY,
    );
}
