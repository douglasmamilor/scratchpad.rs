use crate::framebuffer::FrameBuffer;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Canvas, Texture, TextureAccess, TextureCreator};
use sdl2::video::{Window as SdlWindow, WindowContext};
use sdl2::EventPump;

/// A window wrapper that manages SDL2 rendering
pub struct Window {
    canvas: Canvas<SdlWindow>,
    texture_creator: TextureCreator<WindowContext>,
    texture: Option<Texture<'static>>,
}

impl Window {
    /// Create a new window with the given title and dimensions
    pub fn new(title: &str, width: u32, height: u32) -> Result<(Self, EventPump), String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let texture_creator = canvas.texture_creator();
        
        let event_pump = sdl_context.event_pump()?;

        Ok((
            Self {
                canvas,
                texture_creator,
                texture: None,
            },
            event_pump,
        ))
    }

    /// Update the internal texture to match the framebuffer dimensions
    pub fn update_texture(&mut self, buffer: &FrameBuffer) -> Result<(), String> {
        let (width, height) = (buffer.width() as u32, buffer.height() as u32);
        
        // Check if we need a new texture
        let needs_new = match &self.texture {
            None => true,
            Some(tex) => {
                let query = tex.query();
                query.width != width || query.height != height
            }
        };

        if needs_new {
            // SAFETY: We're creating a new texture with a known lifetime
            // The texture_creator outlives the texture
            let new_texture = unsafe {
                std::mem::transmute::<Texture<'_>, Texture<'static>>(
                    self.texture_creator
                        .create_texture(
                            PixelFormatEnum::ARGB8888,
                            TextureAccess::Streaming,
                            width,
                            height,
                        )
                        .map_err(|e| e.to_string())?
                )
            };
            self.texture = Some(new_texture);
        }

        Ok(())
    }

    /// Present the framebuffer to the screen
    pub fn present(&mut self, buffer: &FrameBuffer) -> Result<(), String> {
        self.update_texture(buffer)?;
        
        if let Some(ref mut texture) = self.texture {
            texture.update(None, buffer.as_bytes(), buffer.pitch())
                .map_err(|e| e.to_string())?;
            self.canvas.copy(texture, None, None)?;
            self.canvas.present();
        }
        
        Ok(())
    }

    /// Clear the window with a specific color
    pub fn clear(&mut self, r: u8, g: u8, b: u8) {
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(r, g, b));
        self.canvas.clear();
    }
}