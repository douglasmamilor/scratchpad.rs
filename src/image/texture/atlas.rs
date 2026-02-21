use std::collections::HashMap;

use crate::{Image, image::PixelFormat};

#[derive(Debug, Clone)]
pub struct AtlasRegion {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

pub struct TextureAtlas {
    image: Image,
    regions: HashMap<String, AtlasRegion>,
    format: PixelFormat,
}

impl TextureAtlas {
    pub fn new(
        images: &[Image],
        format: PixelFormat,
        atlas_width: usize,
        atlas_height: usize,
    ) -> Self {
        assert!(
            !images.is_empty(),
            "TextureAtlas requires at least one image"
        );
        assert!(
            images.iter().all(|img| img.format() == &format),
            "All images must share the same format"
        );
        assert!(
            atlas_width > 0 && atlas_height > 0,
            "Atlas dimensions must be positive"
        );
        assert!(
            images
                .iter()
                .all(|img| img.width() <= atlas_width && img.height() <= atlas_height),
            "An input image is larger than the atlas dimensions"
        );

        let bpp = format.bytes_per_pixel();
        let atlas_stride = atlas_width * bpp;

        let bytes = vec![0u8; atlas_width * atlas_height * bpp];
        let mut atlas = Image::new(atlas_width, atlas_height, bytes, format);

        let mut cursor_x = 0;
        let mut cursor_y = 0;
        let mut row_height = 0;

        let mut regions = HashMap::new();

        for (idx, img) in images.iter().enumerate() {
            if cursor_x + img.width() > atlas_width {
                cursor_x = 0;
                cursor_y += row_height;
                row_height = 0;
            }

            assert!(
                cursor_y + img.height() <= atlas_height,
                "Atlas is too small to fit all images"
            );

            let region = AtlasRegion {
                x: cursor_x,
                y: cursor_y,
                width: img.width(),
                height: img.height(),
            };

            // Copy image data into atlas
            let image_stride = img.width() * bpp;

            for row in 0..img.height() {
                let dst_start = (region.y + row) * atlas_stride + region.x * bpp;
                let src_start = row * image_stride;

                let dst_slice = &mut atlas.data_mut()[dst_start..dst_start + image_stride];
                let src_slice = &img.data()[src_start..src_start + image_stride];

                dst_slice.copy_from_slice(src_slice);
            }

            // Name regions by index; caller can rename after the fact.
            regions.insert(idx.to_string(), region);

            cursor_x += img.width();
            row_height = row_height.max(img.height());
        }

        Self {
            image: atlas,
            regions,
            format,
        }
    }

    /// Returns an immutable reference to the atlas image.
    pub fn image(&self) -> &Image {
        &self.image
    }

    /// Look up a region by name (default names are stringified indices).
    pub fn region(&self, name: &str) -> Option<&AtlasRegion> {
        self.regions.get(name)
    }

    /// All regions in insertion order (by name, region).
    pub fn regions(&self) -> impl Iterator<Item = (&String, &AtlasRegion)> {
        self.regions.iter()
    }

    /// Normalized UV rect for a region (min_u, min_v, max_u, max_v), origin at top-left.
    pub fn uv_rect(&self, name: &str) -> Option<(f32, f32, f32, f32)> {
        let r = self.regions.get(name)?;
        let w = self.image.width() as f32;
        let h = self.image.height() as f32;
        let u0 = r.x as f32 / w;
        let v0 = r.y as f32 / h;
        let u1 = (r.x + r.width) as f32 / w;
        let v1 = (r.y + r.height) as f32 / h;
        Some((u0, v0, u1, v1))
    }
}
