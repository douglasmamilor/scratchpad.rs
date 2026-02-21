use crate::image::Texture as ImageTexture;

/// Texture sampling mode for textured primitives.
#[derive(Debug, Clone, Copy)]
pub enum SamplingMode {
    Nearest,
    Bilinear,
}

/// Re-export image::Texture for renderer users.
///
/// The renderer keeps its own alias so callers can stay within the `renderer`
/// namespace and we can later wrap extra renderer-specific state (e.g. sampling
/// caches) without changing the public API.
pub type Texture = ImageTexture;
