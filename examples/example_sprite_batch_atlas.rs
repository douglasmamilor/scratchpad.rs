use scratchpad_rs::framebuffer::FrameBuffer;
use scratchpad_rs::asset::AssetLoader;
use scratchpad_rs::image::{Image, PixelFormat, Texture, TextureAtlas};
use scratchpad_rs::math::{Mat3, Vec2};
use scratchpad_rs::renderer::{Renderer, SamplingMode, Sprite};
use scratchpad_rs::Color;
use scratchpad_rs::window::Window;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;

const WIDTH: usize = 960;
const HEIGHT: usize = 640;

fn build_atlas() -> (Texture, TextureAtlas, Vec<String>) {
    // Load a few BMP sprites from assets/dinosaurs.
    let sprites = [
        ("trex1", "assets/dinosaurs/trex1.bmp"),
        ("trex2", "assets/dinosaurs/trex2.bmp"),
        ("pterosaur1", "assets/dinosaurs/pterosaur1.bmp"),
        ("dino1", "assets/dinosaurs/dino1.bmp"),
    ];

    let decoded: Vec<(String, Image)> = sprites
        .iter()
        .map(|(name, path)| {
            let img = AssetLoader::load_bmp_image(path).expect("read bmp");
            (name.to_string(), img)
        })
        .collect();

    // Build slices for atlas packing.
    let named_refs: Vec<(&str, &Image)> = decoded
        .iter()
        .map(|(n, img)| (n.as_str(), img))
        .collect();

    // Simple row packing with padding.
    let atlas = TextureAtlas::from_named(&named_refs, PixelFormat::Rgba8, 512, 256, 2);

    let texture = Texture::from(atlas.image());
    let names = decoded.iter().map(|(n, _)| n.clone()).collect::<Vec<_>>();

    (texture, atlas, names)
}

fn sprite_from_region(name: &str, atlas: &TextureAtlas, pos: Vec2, size: Vec2) -> Sprite {
    let (u0, v0, u1, v1) = atlas.uv_rect(name).expect("region not found");
    Sprite::new(pos, size, Vec2::new(u0, v0), Vec2::new(u1, v1))
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) =
        Window::new("Lesson 5.3 - Sprites (Sheets, Batching, Alpha)", WIDTH as u32, HEIGHT as u32)?;

    let mut framebuffer = FrameBuffer::new(WIDTH, HEIGHT);
    window.clear(0, 0, 0);

    let (texture, atlas, names) = build_atlas();
    let mut sampling = SamplingMode::Nearest;

    let mut frame = 0u64;

    println!("Press 'F' to toggle sampling (Nearest/Bilinear), ESC to quit.");

    'running: loop {
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
                    keycode: Some(Keycode::F),
                    ..
                } => {
                    sampling = match sampling {
                        SamplingMode::Nearest => SamplingMode::Bilinear,
                        SamplingMode::Bilinear => SamplingMode::Nearest,
                    };
                    println!("Sampling: {:?}", sampling);
                }
                Event::Window {
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => framebuffer.resize(w as usize, h as usize),
                _ => {}
            }
        }

        {
            let mut renderer = Renderer::new(&mut framebuffer);
            renderer.clear(Color::DARK_GRAY);

            let sprites = [
                sprite_from_region(&names[0], &atlas, Vec2::new(140.0, 120.0), Vec2::new(220.0, 150.0)),
                sprite_from_region(&names[1], &atlas, Vec2::new(380.0, 120.0), Vec2::new(220.0, 150.0)),
                sprite_from_region(&names[2], &atlas, Vec2::new(620.0, 120.0), Vec2::new(160.0, 160.0)),
                sprite_from_region(&names[3], &atlas, Vec2::new(320.0, 320.0), Vec2::new(96.0, 96.0)),
            ];

            renderer.draw_sprite_batch(&sprites, &texture, sampling, Mat3::IDENTITY);
        }

        frame = frame.wrapping_add(1);
        window.present(&framebuffer)?;
    }
}
