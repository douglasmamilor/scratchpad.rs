use scratchpad_rs::Color;
use scratchpad_rs::canvas::Canvas;
use scratchpad_rs::framebuffer::FrameBuffer;
use scratchpad_rs::math::{Mat3, Vec2};
use scratchpad_rs::renderer::Renderer;
use scratchpad_rs::window::Window;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use std::time::Duration;

const INITIAL_WINDOW_WIDTH: usize = 1280;
const INITIAL_WINDOW_HEIGHT: usize = 720;

fn draw_canvas_demo(canvas: &mut Canvas) {
    // Draw a simple house using canvas transforms
    draw_house_with_transforms(canvas);

    // Draw a rotating windmill
    draw_rotating_windmill(canvas);

    // Draw a hierarchical robot arm
    draw_robot_arm(canvas);

    // Draw a pattern using nested transforms
    draw_nested_pattern(canvas);
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) = Window::new(
        "lesson_3_0 - Canvas Transforms Demo",
        INITIAL_WINDOW_WIDTH as u32,
        INITIAL_WINDOW_HEIGHT as u32,
    )?;

    let mut frame_buffer = FrameBuffer::new(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT);

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
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => {
                    // Handle window resize
                    frame_buffer.resize(w as usize, h as usize);
                }
                _ => {}
            }
        }

        {
            let mut renderer = Renderer::new(&mut frame_buffer);
            renderer.clear(Color::RGBA(20, 20, 40, 255));

            let mut canvas = Canvas::new(&mut renderer);
            draw_canvas_demo(&mut canvas);
        }

        // Present the frame
        window.present(&frame_buffer)?;

        // Cap the frame rate
        let frame_time = frame_start.elapsed();
        let target = Duration::from_nanos(1_000_000_000 / 60);
        if let Some(remaining) = target.checked_sub(frame_time) {
            std::thread::sleep(remaining);
        }
    }
}

fn draw_house_with_transforms(canvas: &mut Canvas) {
    // Draw house at different positions using translate
    let positions = [
        Vec2::new(200.0, 200.0),
        Vec2::new(600.0, 200.0),
        Vec2::new(1000.0, 200.0),
    ];

    for pos in positions {
        canvas.with(Mat3::translate(pos.x, pos.y), |canvas| {
            draw_house(canvas);
        });
    }
}

fn draw_house(canvas: &mut Canvas) {
    // House base (rectangle)
    canvas.with(Mat3::scale(1.0, 1.0), |canvas| {
        draw_rect(
            canvas,
            Vec2::new(-40.0, -20.0),
            Vec2::new(40.0, 20.0),
            Color::BROWN,
        );
    });

    // Roof (triangle)
    canvas.draw_line(Vec2::new(-50.0, -20.0), Vec2::new(0.0, -50.0), Color::RED);
    canvas.draw_line(Vec2::new(0.0, -50.0), Vec2::new(50.0, -20.0), Color::RED);
    canvas.draw_line(Vec2::new(50.0, -20.0), Vec2::new(-50.0, -20.0), Color::RED);

    // Door
    canvas.draw_line(
        Vec2::new(-10.0, 20.0),
        Vec2::new(-10.0, 0.0),
        Color::DARK_GRAY,
    );
    canvas.draw_line(
        Vec2::new(-10.0, 0.0),
        Vec2::new(10.0, 0.0),
        Color::DARK_GRAY,
    );
    canvas.draw_line(
        Vec2::new(10.0, 0.0),
        Vec2::new(10.0, 20.0),
        Color::DARK_GRAY,
    );

    // Window
    canvas.draw_line(Vec2::new(20.0, 10.0), Vec2::new(35.0, 10.0), Color::BLUE);
    canvas.draw_line(Vec2::new(35.0, 10.0), Vec2::new(35.0, -5.0), Color::BLUE);
    canvas.draw_line(Vec2::new(35.0, -5.0), Vec2::new(20.0, -5.0), Color::BLUE);
    canvas.draw_line(Vec2::new(20.0, -5.0), Vec2::new(20.0, 10.0), Color::BLUE);
}

fn draw_rotating_windmill(canvas: &mut Canvas) {
    let center = Vec2::new(200.0, 500.0);

    // Windmill base
    canvas.with(Mat3::translate(center.x, center.y), |canvas| {
        // Tower
        canvas.draw_line(Vec2::new(-5.0, 0.0), Vec2::new(-5.0, 80.0), Color::GRAY);
        canvas.draw_line(Vec2::new(5.0, 0.0), Vec2::new(5.0, 80.0), Color::GRAY);
        canvas.draw_line(Vec2::new(-5.0, 80.0), Vec2::new(5.0, 80.0), Color::GRAY);

        // Rotating blades (simulate rotation with different angles)
        let angles = [0.0, 45.0, 90.0, 135.0];
        for (i, angle) in angles.iter().enumerate() {
            canvas.with(Mat3::rotate((*angle as f32).to_radians()), |canvas| {
                // Blade
                canvas.draw_line(
                    Vec2::new(0.0, 0.0),
                    Vec2::new(0.0, 60.0),
                    if i % 2 == 0 {
                        Color::WHITE
                    } else {
                        Color::LIGHT_GRAY
                    },
                );
            });
        }
    });
}

fn draw_robot_arm(canvas: &mut Canvas) {
    let base_pos = Vec2::new(600.0, 500.0);

    canvas.with(Mat3::translate(base_pos.x, base_pos.y), |canvas| {
        // Base
        canvas.draw_line(
            Vec2::new(-10.0, 0.0),
            Vec2::new(10.0, 0.0),
            Color::DARK_GRAY,
        );
        canvas.draw_line(Vec2::new(0.0, 0.0), Vec2::new(0.0, 20.0), Color::DARK_GRAY);

        // First arm segment
        canvas.with(Mat3::rotate(30.0_f32.to_radians()), |canvas| {
            canvas.draw_line(Vec2::new(0.0, 0.0), Vec2::new(0.0, 80.0), Color::BLUE);

            // Second arm segment
            canvas.with(Mat3::translate(0.0, 80.0), |canvas| {
                canvas.with(Mat3::rotate(-45.0_f32.to_radians()), |canvas| {
                    canvas.draw_line(Vec2::new(0.0, 0.0), Vec2::new(0.0, 60.0), Color::GREEN);

                    // Hand/gripper
                    canvas.with(Mat3::translate(0.0, 60.0), |canvas| {
                        canvas.draw_line(Vec2::new(-5.0, 0.0), Vec2::new(5.0, 0.0), Color::RED);
                        canvas.draw_line(Vec2::new(-5.0, 0.0), Vec2::new(-8.0, 10.0), Color::RED);
                        canvas.draw_line(Vec2::new(5.0, 0.0), Vec2::new(8.0, 10.0), Color::RED);
                    });
                });
            });
        });
    });
}

fn draw_nested_pattern(canvas: &mut Canvas) {
    let center = Vec2::new(1000.0, 500.0);

    canvas.with(Mat3::translate(center.x, center.y), |canvas| {
        // Draw a pattern of squares with nested transforms
        for i in 0..8 {
            canvas.with(Mat3::rotate(i as f32 * 45.0_f32.to_radians()), |canvas| {
                canvas.with(
                    Mat3::scale(1.0 - i as f32 * 0.1, 1.0 - i as f32 * 0.1),
                    |canvas| {
                        draw_square(
                            canvas,
                            Color::RGBA(
                                (255 - i * 30) as u8,
                                (100 + i * 20) as u8,
                                (200 - i * 15) as u8,
                                255,
                            ),
                        );
                    },
                );
            });
        }
    });
}

fn draw_square(canvas: &mut Canvas, color: Color) {
    let size = 40.0;
    canvas.draw_line(Vec2::new(-size, -size), Vec2::new(size, -size), color);
    canvas.draw_line(Vec2::new(size, -size), Vec2::new(size, size), color);
    canvas.draw_line(Vec2::new(size, size), Vec2::new(-size, size), color);
    canvas.draw_line(Vec2::new(-size, size), Vec2::new(-size, -size), color);
}

// Helper function to draw rectangles with canvas
fn draw_rect(canvas: &mut Canvas, p0: Vec2, p1: Vec2, color: Color) {
    let x0 = p0.x.min(p1.x);
    let x1 = p0.x.max(p1.x);
    let y0 = p0.y.min(p1.y);
    let y1 = p0.y.max(p1.y);

    // Draw four edges
    canvas.draw_line(Vec2::new(x0, y0), Vec2::new(x1, y0), color); // top
    canvas.draw_line(Vec2::new(x0, y1), Vec2::new(x1, y1), color); // bottom
    canvas.draw_line(Vec2::new(x0, y0), Vec2::new(x0, y1), color); // left
    canvas.draw_line(Vec2::new(x1, y0), Vec2::new(x1, y1), color); // right
}
