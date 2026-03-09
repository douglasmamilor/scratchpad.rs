use scratchpad_rs::asset::AssetLoader;
use scratchpad_rs::framebuffer::FrameBuffer;
use scratchpad_rs::image::{
    adjust_brightness, adjust_contrast, adjust_gamma, adjust_saturation, convolve, grayscale,
    invert, posterize, sepia, sobel_edge_detect, threshold, EdgeMode, Image, Kernel,
};
use scratchpad_rs::math::{Mat3, Vec2};
use scratchpad_rs::renderer::{Renderer, SamplingMode, Sprite, Texture};
use scratchpad_rs::window::Window;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FilterType {
    Original,
    BoxBlur,
    GaussianBlur,
    Sharpen,
    EdgeDetect,
    SobelEdge,
    Emboss,
    Brightness,
    Contrast,
    Saturation,
    Gamma,
    Grayscale,
    Invert,
    Threshold,
    Posterize,
    Sepia,
}

impl FilterType {
    fn next(self) -> Self {
        match self {
            Self::Original => Self::BoxBlur,
            Self::BoxBlur => Self::GaussianBlur,
            Self::GaussianBlur => Self::Sharpen,
            Self::Sharpen => Self::EdgeDetect,
            Self::EdgeDetect => Self::SobelEdge,
            Self::SobelEdge => Self::Emboss,
            Self::Emboss => Self::Brightness,
            Self::Brightness => Self::Contrast,
            Self::Contrast => Self::Saturation,
            Self::Saturation => Self::Gamma,
            Self::Gamma => Self::Grayscale,
            Self::Grayscale => Self::Invert,
            Self::Invert => Self::Threshold,
            Self::Threshold => Self::Posterize,
            Self::Posterize => Self::Sepia,
            Self::Sepia => Self::Original,
        }
    }

    fn prev(self) -> Self {
        match self {
            Self::Original => Self::Sepia,
            Self::BoxBlur => Self::Original,
            Self::GaussianBlur => Self::BoxBlur,
            Self::Sharpen => Self::GaussianBlur,
            Self::EdgeDetect => Self::Sharpen,
            Self::SobelEdge => Self::EdgeDetect,
            Self::Emboss => Self::SobelEdge,
            Self::Brightness => Self::Emboss,
            Self::Contrast => Self::Brightness,
            Self::Saturation => Self::Contrast,
            Self::Gamma => Self::Saturation,
            Self::Grayscale => Self::Gamma,
            Self::Invert => Self::Grayscale,
            Self::Threshold => Self::Invert,
            Self::Posterize => Self::Threshold,
            Self::Sepia => Self::Posterize,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Original => "Original",
            Self::BoxBlur => "Box Blur 5x5",
            Self::GaussianBlur => "Gaussian Blur 5x5",
            Self::Sharpen => "Sharpen",
            Self::EdgeDetect => "Edge Detect (Laplacian)",
            Self::SobelEdge => "Sobel Edge Detection",
            Self::Emboss => "Emboss",
            Self::Brightness => "Brightness +50",
            Self::Contrast => "Contrast x1.5",
            Self::Saturation => "Saturation x1.8",
            Self::Gamma => "Gamma 1.5",
            Self::Grayscale => "Grayscale",
            Self::Invert => "Invert",
            Self::Threshold => "Threshold 128",
            Self::Posterize => "Posterize (4 levels)",
            Self::Sepia => "Sepia",
        }
    }
}

fn apply_filter(image: &Image, filter: FilterType) -> Image {
    match filter {
        FilterType::Original => {
            // Clone the image by creating a new one with same data
            Image::new(
                image.width(),
                image.height(),
                image.data().to_vec(),
                *image.format(),
            )
        }
        FilterType::BoxBlur => convolve(image, &Kernel::box_blur_5x5(), EdgeMode::Clamp),
        FilterType::GaussianBlur => convolve(image, &Kernel::gaussian_5x5(), EdgeMode::Clamp),
        FilterType::Sharpen => convolve(image, &Kernel::sharpen(), EdgeMode::Clamp),
        FilterType::EdgeDetect => convolve(image, &Kernel::edge_detect(), EdgeMode::Clamp),
        FilterType::SobelEdge => sobel_edge_detect(image, EdgeMode::Clamp),
        FilterType::Emboss => convolve(image, &Kernel::emboss(), EdgeMode::Clamp),
        FilterType::Brightness => adjust_brightness(image, 50),
        FilterType::Contrast => adjust_contrast(image, 1.5),
        FilterType::Saturation => adjust_saturation(image, 1.8),
        FilterType::Gamma => adjust_gamma(image, 1.5),
        FilterType::Grayscale => grayscale(image),
        FilterType::Invert => invert(image),
        FilterType::Threshold => threshold(image, 128),
        FilterType::Posterize => posterize(image, 4),
        FilterType::Sepia => sepia(image),
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut event_pump) =
        Window::new("Lesson 5.4 - Image Filters", WIDTH as u32, HEIGHT as u32)?;

    let mut framebuffer = FrameBuffer::new(WIDTH, HEIGHT);
    window.clear(0, 0, 0);

    // Load source image
    let source_image = AssetLoader::load_bmp_image("assets/people/free1.bmp")?;

    let mut current_filter = FilterType::Original;
    let mut filtered_image = apply_filter(&source_image, current_filter);
    let mut texture = Texture::from(&filtered_image);

    // Sprite to display the filtered image (scaled to fit window)
    let sprite = Sprite::new(
        Vec2::new(40.0, 60.0),
        Vec2::new(600.0, 600.0),
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 1.0),
    );

    // Original image sprite for comparison
    let original_texture = Texture::from(&source_image);
    let original_sprite = Sprite::new(
        Vec2::new(680.0, 60.0),
        Vec2::new(560.0, 560.0),
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 1.0),
    );

    println!("Image Filter Demo");
    println!("  Left/Right: Change filter");
    println!("  Esc: Quit");
    println!();
    println!("Current: {}", current_filter.name());

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
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    current_filter = current_filter.next();
                    filtered_image = apply_filter(&source_image, current_filter);
                    texture = Texture::from(&filtered_image);
                    println!("Filter: {}", current_filter.name());
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    current_filter = current_filter.prev();
                    filtered_image = apply_filter(&source_image, current_filter);
                    texture = Texture::from(&filtered_image);
                    println!("Filter: {}", current_filter.name());
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
            renderer.clear(scratchpad_rs::Color::DARK_GRAY);

            // Draw filtered image (left)
            renderer.draw_sprite(sprite, &texture, SamplingMode::Bilinear, Mat3::IDENTITY);

            // Draw original image (right) for comparison
            renderer.draw_sprite(
                original_sprite,
                &original_texture,
                SamplingMode::Bilinear,
                Mat3::IDENTITY,
            );
        }

        window.present(&framebuffer)?;
    }
}
