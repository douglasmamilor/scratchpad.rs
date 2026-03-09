use scratchpad_rs::animation::{ease_in_out_sine, KeyFrameAnimation, Keyframe};
use scratchpad_rs::asset::AssetLoader;
use scratchpad_rs::framebuffer::FrameBuffer;
use scratchpad_rs::image::{PixelFormat, TextureAtlas};
use scratchpad_rs::math::{Mat3, Vec2};
use scratchpad_rs::renderer::{Renderer, SamplingMode, Sprite, Texture};
use scratchpad_rs::window::Window;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use std::time::Duration;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

// Time of day phases (in seconds of animation time)
const PHASE_NIGHT: f32 = 0.0;
const PHASE_SUNRISE: f32 = 4.0;
const PHASE_NOON: f32 = 8.0;
const PHASE_AFTERNOON: f32 = 12.0;
const PHASE_SUNSET: f32 = 16.0;
const PHASE_END: f32 = 20.0; // loops back to night

// Arc parameters for celestial bodies
const ARC_CENTER_X: f32 = WIDTH as f32 / 2.0;
const ARC_CENTER_Y: f32 = HEIGHT as f32 + 100.0; // below screen for arc pivot
const ARC_RADIUS: f32 = 500.0;
const CELESTIAL_SIZE: f32 = 80.0;

fn arc_position(angle_deg: f32) -> Vec2 {
    let angle_rad = angle_deg.to_radians();
    Vec2::new(
        ARC_CENTER_X + ARC_RADIUS * angle_rad.cos(),
        ARC_CENTER_Y - ARC_RADIUS * angle_rad.sin(),
    )
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TimeOfDay {
    Night,
    Sunrise,
    Noon,
    Afternoon,
    Sunset,
}

fn get_time_of_day(time: f32) -> TimeOfDay {
    if time < PHASE_SUNRISE {
        TimeOfDay::Night
    } else if time < PHASE_NOON {
        TimeOfDay::Sunrise
    } else if time < PHASE_AFTERNOON {
        TimeOfDay::Noon
    } else if time < PHASE_SUNSET {
        TimeOfDay::Afternoon
    } else {
        TimeOfDay::Sunset
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) = Window::new(
        "Lesson 5.3 - Day/Night Cycle (Sprite Animation)",
        WIDTH as u32,
        HEIGHT as u32,
    )?;

    let mut framebuffer = FrameBuffer::new(WIDTH, HEIGHT);
    window.clear(0, 0, 0);

    // Load all sprite images
    let house_img = AssetLoader::load_bmp_image("assets/generated/house_bottom_up.bmp")?;
    let tree_img = AssetLoader::load_bmp_image("assets/generated/tree_bottom_up.bmp")?;
    let sun_img = AssetLoader::load_bmp_image("assets/generated/sun.bmp")?;
    let moon_img = AssetLoader::load_bmp_image("assets/generated/moon.bmp")?;

    // Load background scene images
    let scene_night = AssetLoader::load_bmp_image("assets/generated/scene_night.bmp")?;
    let scene_sunrise = AssetLoader::load_bmp_image("assets/generated/scene_sunrise.bmp")?;
    let scene_noon = AssetLoader::load_bmp_image("assets/generated/scene_noon.bmp")?;
    let scene_afternoon = AssetLoader::load_bmp_image("assets/generated/scene_afternoon.bmp")?;
    let scene_sunset = AssetLoader::load_bmp_image("assets/generated/scene_sunset.bmp")?;

    // Build texture atlas with all sprites
    let named = [
        ("house", &house_img),
        ("tree", &tree_img),
        ("sun", &sun_img),
        ("moon", &moon_img),
        ("scene_night", &scene_night),
        ("scene_sunrise", &scene_sunrise),
        ("scene_noon", &scene_noon),
        ("scene_afternoon", &scene_afternoon),
        ("scene_sunset", &scene_sunset),
    ];

    let atlas = TextureAtlas::from_named(&named, PixelFormat::Rgb8, 2048, 2048, 2);
    let texture = Texture::from(atlas.image());

    // Get UV rectangles for all sprites
    let house_uv = atlas.uv_rect("house").expect("house UVs");
    let tree_uv = atlas.uv_rect("tree").expect("tree UVs");
    let sun_uv = atlas.uv_rect("sun").expect("sun UVs");
    let moon_uv = atlas.uv_rect("moon").expect("moon UVs");
    let night_uv = atlas.uv_rect("scene_night").expect("night UVs");
    let sunrise_uv = atlas.uv_rect("scene_sunrise").expect("sunrise UVs");
    let noon_uv = atlas.uv_rect("scene_noon").expect("noon UVs");
    let afternoon_uv = atlas.uv_rect("scene_afternoon").expect("afternoon UVs");
    let sunset_uv = atlas.uv_rect("scene_sunset").expect("sunset UVs");

    // Static sprites for house and tree (foreground)
    let house_sprite = Sprite::new(
        Vec2::new(200.0, 280.0),
        Vec2::new(320.0, 320.0),
        Vec2::new(house_uv.0, house_uv.1),
        Vec2::new(house_uv.2, house_uv.3),
    );
    let tree_sprite = Sprite::new(
        Vec2::new(760.0, 280.0),
        Vec2::new(320.0, 320.0),
        Vec2::new(tree_uv.0, tree_uv.1),
        Vec2::new(tree_uv.2, tree_uv.3),
    );

    // Background sprite (covers full screen, stretched)
    let bg_pos = Vec2::new(0.0, 0.0);
    let bg_size = Vec2::new(WIDTH as f32, HEIGHT as f32);

    // Sun animation: rises from left (135 deg), peaks at noon (90 deg), sets right (45 deg)
    // During night phases, sun is below horizon
    let mut sun_anim = KeyFrameAnimation::new();
    sun_anim.add_keyframes(vec![
        Keyframe::with_easing(PHASE_NIGHT, arc_position(180.0), ease_in_out_sine), // below left
        Keyframe::with_easing(PHASE_SUNRISE, arc_position(135.0), ease_in_out_sine), // rising
        Keyframe::with_easing(PHASE_NOON, arc_position(90.0), ease_in_out_sine),     // peak
        Keyframe::with_easing(PHASE_AFTERNOON, arc_position(60.0), ease_in_out_sine), // descending
        Keyframe::with_easing(PHASE_SUNSET, arc_position(30.0), ease_in_out_sine),   // setting
        Keyframe::new(PHASE_END, arc_position(0.0)),                                  // below right
    ]);
    sun_anim.set_looping(true);
    sun_anim.play();

    // Moon animation: opposite phase to sun
    // Moon is visible during night, sets as sun rises
    let mut moon_anim = KeyFrameAnimation::new();
    moon_anim.add_keyframes(vec![
        Keyframe::with_easing(PHASE_NIGHT, arc_position(90.0), ease_in_out_sine), // peak at night
        Keyframe::with_easing(PHASE_SUNRISE, arc_position(45.0), ease_in_out_sine), // setting
        Keyframe::with_easing(PHASE_NOON, arc_position(0.0), ease_in_out_sine),   // below horizon
        Keyframe::with_easing(PHASE_AFTERNOON, arc_position(-30.0), ease_in_out_sine),
        Keyframe::with_easing(PHASE_SUNSET, arc_position(135.0), ease_in_out_sine), // rising again
        Keyframe::new(PHASE_END, arc_position(90.0)),                               // back to peak
    ]);
    moon_anim.set_looping(true);
    moon_anim.play();

    let sampling = SamplingMode::Bilinear;
    let mut paused = false;
    let mut time_scale = 1.0f32;

    println!("Day/Night Cycle Animation");
    println!("  Space: Pause/Resume");
    println!("  Up/Down: Speed up/slow down");
    println!("  R: Reset");
    println!("  Esc: Quit");

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

                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    paused = !paused;
                    if paused {
                        sun_anim.pause();
                        moon_anim.pause();
                        println!("Paused");
                    } else {
                        sun_anim.play();
                        moon_anim.play();
                        println!("Resumed");
                    }
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    time_scale = (time_scale * 1.5).min(8.0);
                    println!("Speed: {:.1}x", time_scale);
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    time_scale = (time_scale / 1.5).max(0.25);
                    println!("Speed: {:.1}x", time_scale);
                }

                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    sun_anim.stop();
                    moon_anim.stop();
                    sun_anim.play();
                    moon_anim.play();
                    println!("Reset");
                }

                Event::Window {
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => framebuffer.resize(w as usize, h as usize),

                _ => {}
            }
        }

        // Update animations
        if !paused {
            let dt = (1.0 / 60.0) * time_scale;
            sun_anim.update(dt);
            moon_anim.update(dt);
        }

        // Get current animation values
        let sun_pos = sun_anim.value();
        let moon_pos = moon_anim.value();
        let current_time = sun_anim.current_time();
        let time_of_day = get_time_of_day(current_time % PHASE_END);

        // Select background UVs based on time of day
        let bg_uv = match time_of_day {
            TimeOfDay::Night => night_uv,
            TimeOfDay::Sunrise => sunrise_uv,
            TimeOfDay::Noon => noon_uv,
            TimeOfDay::Afternoon => afternoon_uv,
            TimeOfDay::Sunset => sunset_uv,
        };

        // Create dynamic sprites
        let bg_sprite = Sprite::new(
            bg_pos,
            bg_size,
            Vec2::new(bg_uv.0, bg_uv.1),
            Vec2::new(bg_uv.2, bg_uv.3),
        );

        let sun_sprite = Sprite::new(
            sun_pos - Vec2::new(CELESTIAL_SIZE / 2.0, CELESTIAL_SIZE / 2.0),
            Vec2::new(CELESTIAL_SIZE, CELESTIAL_SIZE),
            Vec2::new(sun_uv.0, sun_uv.1),
            Vec2::new(sun_uv.2, sun_uv.3),
        );

        let moon_sprite = Sprite::new(
            moon_pos - Vec2::new(CELESTIAL_SIZE / 2.0, CELESTIAL_SIZE / 2.0),
            Vec2::new(CELESTIAL_SIZE, CELESTIAL_SIZE),
            Vec2::new(moon_uv.0, moon_uv.1),
            Vec2::new(moon_uv.2, moon_uv.3),
        );

        // Render scene
        {
            let mut renderer = Renderer::new(&mut framebuffer);

            // Layer 1: Background
            renderer.draw_sprite(bg_sprite, &texture, sampling, Mat3::IDENTITY);

            // Layer 2: Celestial bodies (draw moon first so sun overlaps during transitions)
            // Only draw if above horizon (y < HEIGHT)
            if moon_pos.y < HEIGHT as f32 {
                renderer.draw_sprite(moon_sprite, &texture, sampling, Mat3::IDENTITY);
            }
            if sun_pos.y < HEIGHT as f32 {
                renderer.draw_sprite(sun_sprite, &texture, sampling, Mat3::IDENTITY);
            }

            // Layer 3: Foreground (house and tree)
            renderer.draw_sprite(house_sprite, &texture, sampling, Mat3::IDENTITY);
            renderer.draw_sprite(tree_sprite, &texture, sampling, Mat3::IDENTITY);
        }

        window.present(&framebuffer)?;

        // Frame rate limiting (60 FPS)
        let frame_time = frame_start.elapsed();
        let target_frame_time = Duration::from_secs_f32(1.0 / 60.0);
        if frame_time < target_frame_time {
            std::thread::sleep(target_frame_time - frame_time);
        }
    }
}
