use scratchpad_rs::Color;
use scratchpad_rs::framebuffer::FrameBuffer;
use scratchpad_rs::math::{Mat3, Vec2};
use scratchpad_rs::renderer::Renderer;
use scratchpad_rs::window::Window;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

fn draw_demo(renderer: &mut Renderer, aa: bool) {
    renderer.clear(Color::DARK_GRAY);

    // Toggle AA state.
    renderer.set_triangle_aa_enabled(aa);
    renderer.set_triangle_aa_supersample(true);
    renderer.set_triangle_aa_gamma(true);

    // One large triangle centered-ish on screen.
    renderer.fill_triangle(
        Vec2::new(200.0, 150.0),
        Vec2::new(1080.0, 200.0),
        Vec2::new(400.0, 620.0),
        Color::CYAN,
        Mat3::IDENTITY,
    );
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) = Window::new(
        "Lesson 4.5 - Anti-Aliasing (Aliased vs AA Triangles)",
        WIDTH as u32,
        HEIGHT as u32,
    )?;

    let mut framebuffer = FrameBuffer::new(WIDTH, HEIGHT);
    window.clear(0, 0, 0);

    let mut aa_on = false;

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
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    aa_on = !aa_on;
                }
                Event::Window {
                    win_event: WindowEvent::Resized(width, height),
                    ..
                } => framebuffer.resize(width as usize, height as usize),
                _ => {}
            }
        }

        {
            let mut renderer = Renderer::new(&mut framebuffer);
            draw_demo(&mut renderer, aa_on);
        }

        window.present(&framebuffer)?;
    }
}
