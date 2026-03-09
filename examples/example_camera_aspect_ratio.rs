use scratchpad_rs::Color;
use scratchpad_rs::camera::Camera;
use scratchpad_rs::framebuffer::FrameBuffer;
use scratchpad_rs::math::{Mat3, Vec2};
use scratchpad_rs::math::{Point2, Rect};
use scratchpad_rs::renderer::Renderer;
use scratchpad_rs::window::Window;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use std::time::Duration;

const INITIAL_WINDOW_WIDTH: usize = 1280;
const INITIAL_WINDOW_HEIGHT: usize = 720;

// Common aspect ratios
const ASPECT_RATIOS: &[(f32, &str)] = &[
    (16.0 / 9.0, "16:9 (HD/4K)"),
    (4.0 / 3.0, "4:3 (Classic)"),
    (21.0 / 9.0, "21:9 (Ultrawide)"),
    (1.0, "1:1 (Square)"),
    (3.0 / 4.0, "3:4 (Portrait)"),
    (9.0 / 16.0, "9:16 (Vertical)"),
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) = Window::new(
        "lesson_3_1_aspect_ratio - Letterboxing & Pillarboxing",
        INITIAL_WINDOW_WIDTH as u32,
        INITIAL_WINDOW_HEIGHT as u32,
    )?;

    let mut frame_buffer = FrameBuffer::new(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT);

    // Initial viewport (fullscreen)
    let initial_viewport = Rect::new(
        0.0,
        0.0,
        INITIAL_WINDOW_WIDTH as f32,
        INITIAL_WINDOW_HEIGHT as f32,
    );
    let mut camera = Camera::default(initial_viewport);

    // Aspect ratio state
    let mut aspect_index = 0; // Start with 16:9
    let mut show_borders = true;
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
                } => return Ok(()),

                Event::Window {
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => {
                    frame_buffer.resize(w as usize, h as usize);
                    // Recalculate viewport with current aspect ratio
                    let target_aspect = ASPECT_RATIOS[aspect_index].0;
                    let fitted_viewport =
                        camera.calculate_fitted_viewport(target_aspect, w as usize, h as usize);
                    camera.set_viewport(fitted_viewport);
                }

                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    match key {
                        // Cycle through aspect ratios
                        Keycode::Right | Keycode::Space => {
                            aspect_index = (aspect_index + 1) % ASPECT_RATIOS.len();
                            let target_aspect = ASPECT_RATIOS[aspect_index].0;
                            let fitted_viewport = camera.calculate_fitted_viewport(
                                target_aspect,
                                frame_buffer.width(),
                                frame_buffer.height(),
                            );
                            camera.set_viewport(fitted_viewport);
                        }
                        Keycode::Left => {
                            aspect_index = if aspect_index == 0 {
                                ASPECT_RATIOS.len() - 1
                            } else {
                                aspect_index - 1
                            };
                            let target_aspect = ASPECT_RATIOS[aspect_index].0;
                            let fitted_viewport = camera.calculate_fitted_viewport(
                                target_aspect,
                                frame_buffer.width(),
                                frame_buffer.height(),
                            );
                            camera.set_viewport(fitted_viewport);
                        }

                        // Toggle viewport border
                        Keycode::B => show_borders = !show_borders,

                        // Toggle grid
                        Keycode::G => show_grid = !show_grid,

                        _ => {}
                    }
                }

                _ => {}
            }
        }

        // Render
        {
            // Get dimensions before creating renderer to avoid borrowing issues
            let fb_width = frame_buffer.width();
            let fb_height = frame_buffer.height();

            let mut renderer = Renderer::new(&mut frame_buffer);

            // Clear entire framebuffer to dark gray (this will be visible in letterbox/pillarbox areas)
            renderer.clear(Color::RGBA(20, 20, 30, 255));

            // Draw content in the viewport
            let viewport = camera.viewport();
            draw_viewport_content(&mut renderer, &camera, viewport, show_grid);

            // Draw viewport border if enabled
            if show_borders {
                draw_viewport_border(&mut renderer, viewport);
            }

            // Draw letterbox/pillarbox areas (darker)
            draw_letterbox_pillarbox(&mut renderer, viewport, fb_width, fb_height);

            // Draw info
            draw_aspect_ratio_info(&mut renderer, &camera, aspect_index, fb_width, fb_height);
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

fn draw_viewport_content(
    renderer: &mut Renderer,
    camera: &Camera,
    viewport: Rect,
    show_grid: bool,
) {
    // Draw content that maintains aspect ratio within the viewport
    // This content should look the same regardless of letterboxing/pillarboxing

    // Draw a grid in the viewport
    if show_grid {
        draw_viewport_grid(renderer, camera, viewport);
    }

    // Draw a centered circle that fits within the viewport
    let center = Vec2::new(
        viewport.x + viewport.width / 2.0,
        viewport.y + viewport.height / 2.0,
    );
    let radius = viewport.width.min(viewport.height) * 0.15;
    renderer.draw_circle(center, radius, Color::CYAN, Mat3::IDENTITY);

    // Draw a smaller filled circle inside
    renderer.fill_ellipse(
        center,
        radius * 0.7,
        radius * 0.7,
        Color::RGBA(0, 200, 255, 150),
        Mat3::IDENTITY,
    );

    // Draw coordinate axes centered
    let axis_length = viewport.width.min(viewport.height) * 0.2;

    // X-axis (red)
    renderer.draw_line_aa(
        Vec2::new(center.x - axis_length, center.y),
        Vec2::new(center.x + axis_length, center.y),
        Color::RED,
        Mat3::IDENTITY,
    );

    // Y-axis (green)
    renderer.draw_line_aa(
        Vec2::new(center.x, center.y - axis_length),
        Vec2::new(center.x, center.y + axis_length),
        Color::GREEN,
        Mat3::IDENTITY,
    );

    // Draw corner markers to show viewport bounds
    let corner_size = 30.0;
    let corners = [
        Vec2::new(viewport.x + corner_size, viewport.y + corner_size), // Top-left
        Vec2::new(
            viewport.x + viewport.width - corner_size,
            viewport.y + corner_size,
        ), // Top-right
        Vec2::new(
            viewport.x + viewport.width - corner_size,
            viewport.y + viewport.height - corner_size,
        ), // Bottom-right
        Vec2::new(
            viewport.x + corner_size,
            viewport.y + viewport.height - corner_size,
        ), // Bottom-left
    ];

    for corner in &corners {
        // Draw corner brackets
        let bracket_len = 15.0;

        // Horizontal line
        renderer.draw_line_aa(
            Vec2::new(corner.x - bracket_len, corner.y),
            Vec2::new(corner.x, corner.y),
            Color::YELLOW,
            Mat3::IDENTITY,
        );

        // Vertical line
        renderer.draw_line_aa(
            Vec2::new(corner.x, corner.y - bracket_len),
            Vec2::new(corner.x, corner.y),
            Color::YELLOW,
            Mat3::IDENTITY,
        );
    }
}

fn draw_viewport_grid(renderer: &mut Renderer, camera: &Camera, viewport: Rect) {
    let grid_color = Color::RGBA(80, 80, 100, 255);
    let grid_spacing = 50.0;

    // Convert grid lines from world space to screen space
    let top_left_world = camera.screen_to_world(Point2::new(viewport.x, viewport.y));
    let bottom_right_world = camera.screen_to_world(Point2::new(
        viewport.x + viewport.width,
        viewport.y + viewport.height,
    ));

    // Draw vertical grid lines
    let start_x = (top_left_world.x / grid_spacing).floor() * grid_spacing;
    let end_x = (bottom_right_world.x / grid_spacing).ceil() * grid_spacing;

    for x in (start_x as i32)..=(end_x as i32) {
        let x_f32 = x as f32;
        if (x_f32 % grid_spacing).abs() < 1.0 {
            let start_screen = camera.world_to_screen(Point2::new(x_f32, top_left_world.y));
            let end_screen = camera.world_to_screen(Point2::new(x_f32, bottom_right_world.y));

            // Clip to viewport bounds
            if start_screen.x >= viewport.x && start_screen.x <= viewport.x + viewport.width {
                renderer.draw_line_aa(
                    Vec2::new(
                        start_screen.x.max(viewport.x),
                        start_screen.y.max(viewport.y),
                    ),
                    Vec2::new(
                        end_screen.x.max(viewport.x),
                        end_screen.y.min(viewport.y + viewport.height),
                    ),
                    grid_color,
                    Mat3::IDENTITY,
                );
            }
        }
    }

    // Draw horizontal grid lines
    let start_y = (top_left_world.y / grid_spacing).floor() * grid_spacing;
    let end_y = (bottom_right_world.y / grid_spacing).ceil() * grid_spacing;

    for y in (start_y as i32)..=(end_y as i32) {
        let y_f32 = y as f32;
        if (y_f32 % grid_spacing).abs() < 1.0 {
            let start_screen = camera.world_to_screen(Point2::new(top_left_world.x, y_f32));
            let end_screen = camera.world_to_screen(Point2::new(bottom_right_world.x, y_f32));

            // Clip to viewport bounds
            if start_screen.y >= viewport.y && start_screen.y <= viewport.y + viewport.height {
                renderer.draw_line_aa(
                    Vec2::new(
                        start_screen.x.max(viewport.x),
                        start_screen.y.max(viewport.y),
                    ),
                    Vec2::new(
                        end_screen.x.min(viewport.x + viewport.width),
                        end_screen.y.max(viewport.y),
                    ),
                    grid_color,
                    Mat3::IDENTITY,
                );
            }
        }
    }
}

fn draw_viewport_border(renderer: &mut Renderer, viewport: Rect) {
    // Draw bright border around viewport
    let border_color = Color::WHITE;

    // Top edge
    renderer.draw_line_aa(
        Vec2::new(viewport.x, viewport.y),
        Vec2::new(viewport.x + viewport.width, viewport.y),
        border_color,
        Mat3::IDENTITY,
    );

    // Bottom edge
    renderer.draw_line_aa(
        Vec2::new(viewport.x, viewport.y + viewport.height),
        Vec2::new(viewport.x + viewport.width, viewport.y + viewport.height),
        border_color,
        Mat3::IDENTITY,
    );

    // Left edge
    renderer.draw_line_aa(
        Vec2::new(viewport.x, viewport.y),
        Vec2::new(viewport.x, viewport.y + viewport.height),
        border_color,
        Mat3::IDENTITY,
    );

    // Right edge
    renderer.draw_line_aa(
        Vec2::new(viewport.x + viewport.width, viewport.y),
        Vec2::new(viewport.x + viewport.width, viewport.y + viewport.height),
        border_color,
        Mat3::IDENTITY,
    );
}

fn draw_letterbox_pillarbox(
    renderer: &mut Renderer,
    viewport: Rect,
    framebuffer_width: usize,
    framebuffer_height: usize,
) {
    // Draw darker areas outside the viewport (letterbox/pillarbox regions)
    let bar_color = Color::RGBA(10, 10, 15, 255);
    let fb_width = framebuffer_width as f32;
    let fb_height = framebuffer_height as f32;

    // Top letterbox area
    if viewport.y > 0.0 {
        renderer.fill_rect(
            Vec2::new(0.0, 0.0),
            Vec2::new(fb_width, viewport.y),
            bar_color,
            Mat3::IDENTITY,
        );
    }

    // Bottom letterbox area
    if viewport.y + viewport.height < fb_height {
        renderer.fill_rect(
            Vec2::new(0.0, viewport.y + viewport.height),
            Vec2::new(fb_width, fb_height),
            bar_color,
            Mat3::IDENTITY,
        );
    }

    // Left pillarbox area
    if viewport.x > 0.0 {
        renderer.fill_rect(
            Vec2::new(0.0, viewport.y.max(0.0)),
            Vec2::new(viewport.x, (viewport.y + viewport.height).min(fb_height)),
            bar_color,
            Mat3::IDENTITY,
        );
    }

    // Right pillarbox area
    if viewport.x + viewport.width < fb_width {
        renderer.fill_rect(
            Vec2::new(viewport.x + viewport.width, viewport.y.max(0.0)),
            Vec2::new(fb_width, (viewport.y + viewport.height).min(fb_height)),
            bar_color,
            Mat3::IDENTITY,
        );
    }
}

fn draw_aspect_ratio_info(
    renderer: &mut Renderer,
    camera: &Camera,
    aspect_index: usize,
    fb_width: usize,
    fb_height: usize,
) {
    // Draw info in top-left corner (outside viewport, so always visible)
    let info_x = 10.0;
    let info_y = 10.0;

    // Draw semi-transparent background
    let panel_width = 350.0;
    let panel_height = 180.0;
    renderer.fill_rect(
        Vec2::new(info_x, info_y),
        Vec2::new(info_x + panel_width, info_y + panel_height),
        Color::RGBA(0, 0, 0, 200),
        Mat3::IDENTITY,
    );

    // Draw info text would go here when text rendering is available
    // For now, we'll draw visual indicators

    // Draw aspect ratio indicator bars
    let (target_aspect, _aspect_name) = ASPECT_RATIOS[aspect_index];
    let bar_x = info_x + 20.0;
    let bar_y = info_y + 40.0;
    let bar_width = 300.0;
    let bar_height = 20.0;

    // Draw target aspect ratio as a rectangle
    let indicator_height = if target_aspect >= 1.0 {
        // Landscape: width determines size
        bar_width / target_aspect
    } else {
        // Portrait: height determines size
        bar_width * target_aspect
    };

    let indicator_y = bar_y + (bar_height - indicator_height) / 2.0;

    renderer.fill_rect(
        Vec2::new(bar_x, indicator_y),
        Vec2::new(bar_x + bar_width, indicator_y + indicator_height),
        Color::CYAN,
        Mat3::IDENTITY,
    );

    // Draw framebuffer aspect ratio for comparison
    let fb_aspect = fb_width as f32 / fb_height as f32;
    let fb_indicator_height = if fb_aspect >= 1.0 {
        bar_width / fb_aspect
    } else {
        bar_width * fb_aspect
    };

    let fb_indicator_y = bar_y + bar_height + 10.0 + (bar_height - fb_indicator_height) / 2.0;

    renderer.fill_rect(
        Vec2::new(bar_x, fb_indicator_y),
        Vec2::new(bar_x + bar_width, fb_indicator_y + fb_indicator_height),
        Color::YELLOW,
        Mat3::IDENTITY,
    );

    // Draw viewport dimensions indicator
    let viewport = camera.viewport();
    let viewport_indicator_height = if viewport.width / viewport.height >= 1.0 {
        bar_width / (viewport.width / viewport.height)
    } else {
        bar_width * (viewport.width / viewport.height)
    };

    let viewport_indicator_y =
        fb_indicator_y + bar_height + 10.0 + (bar_height - viewport_indicator_height) / 2.0;

    renderer.fill_rect(
        Vec2::new(bar_x, viewport_indicator_y),
        Vec2::new(
            bar_x + bar_width,
            viewport_indicator_y + viewport_indicator_height,
        ),
        Color::GREEN,
        Mat3::IDENTITY,
    );

    // Draw labels using simple shapes (until text rendering is available)
    // These would be text labels in the final version
    let label_size = 5.0;
    let label_spacing = 25.0;

    // Target aspect label (cyan dot)
    renderer.fill_rect(
        Vec2::new(bar_x - label_spacing, bar_y + bar_height / 2.0 - label_size),
        Vec2::new(
            bar_x - label_spacing + label_size * 2.0,
            bar_y + bar_height / 2.0 + label_size,
        ),
        Color::CYAN,
        Mat3::IDENTITY,
    );

    // Framebuffer label (yellow dot)
    renderer.fill_rect(
        Vec2::new(
            bar_x - label_spacing,
            fb_indicator_y + bar_height / 2.0 - label_size,
        ),
        Vec2::new(
            bar_x - label_spacing + label_size * 2.0,
            fb_indicator_y + bar_height / 2.0 + label_size,
        ),
        Color::YELLOW,
        Mat3::IDENTITY,
    );

    // Viewport label (green dot)
    renderer.fill_rect(
        Vec2::new(
            bar_x - label_spacing,
            viewport_indicator_y + bar_height / 2.0 - label_size,
        ),
        Vec2::new(
            bar_x - label_spacing + label_size * 2.0,
            viewport_indicator_y + bar_height / 2.0 + label_size,
        ),
        Color::GREEN,
        Mat3::IDENTITY,
    );

    // Draw mode indicator
    let mode_text_y = info_y + panel_height - 30.0;
    // This would display: "Mode: {aspect_name}" when text rendering is available
    // Visual placeholder for now
    renderer.fill_rect(
        Vec2::new(bar_x, mode_text_y),
        Vec2::new(bar_x + 100.0, mode_text_y + 15.0),
        Color::WHITE,
        Mat3::IDENTITY,
    );

    // Draw controls hint
    // This would display controls when text rendering is available
    // Controls: Left/Right or Space to cycle aspect ratios, B to toggle border, G to toggle grid
}
