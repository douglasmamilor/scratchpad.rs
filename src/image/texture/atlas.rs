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
    /// Pack a list of images (auto-named by index) into a simple row-major atlas with no padding.
    pub fn new(
        images: &[Image],
        format: PixelFormat,
        atlas_width: usize,
        atlas_height: usize,
    ) -> Self {
        let named: Vec<(String, &Image)> = images
            .iter()
            .enumerate()
            .map(|(i, img)| (i.to_string(), img))
            .collect();
        let named_refs: Vec<(&str, &Image)> = named
            .iter()
            .map(|(name, img)| (name.as_str(), *img))
            .collect();
        Self::from_named(&named_refs, format, atlas_width, atlas_height, 0)
    }

    /// Pack named images into a row-major atlas with optional padding between sprites.
    ///
    /// Returns a new `TextureAtlas` containing the merged image and region metadata.
    pub fn from_named(
        images: &[(&str, &Image)],
        format: PixelFormat,
        atlas_width: usize,
        atlas_height: usize,
        padding: usize,
    ) -> Self {
        assert!(
            !images.is_empty(),
            "TextureAtlas requires at least one image"
        );
        assert!(
            images.iter().all(|(_, img)| img.format() == &format),
            "All images must share the same format"
        );
        assert!(
            atlas_width > 0 && atlas_height > 0,
            "Atlas dimensions must be positive"
        );
        assert!(
            images
                .iter()
                .all(|(_, img)| img.width() <= atlas_width && img.height() <= atlas_height),
            "An input image is larger than the atlas dimensions"
        );

        let bpp = format.bytes_per_pixel();
        let atlas_stride = atlas_width * bpp;

        let bytes = vec![0u8; atlas_width * atlas_height * bpp];
        let mut atlas_img = Image::new(atlas_width, atlas_height, bytes, format);

        let mut cursor_x: usize = 0;
        let mut cursor_y: usize = 0;
        let mut row_height: usize = 0;

        let mut regions = HashMap::new();

        for (name, img) in images.iter() {
            if cursor_x + img.width() > atlas_width {
                cursor_x = 0;
                cursor_y = cursor_y.saturating_add(row_height).saturating_add(padding);
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

                let dst_slice = &mut atlas_img.data_mut()[dst_start..dst_start + image_stride];
                let src_slice = &img.data()[src_start..src_start + image_stride];

                dst_slice.copy_from_slice(src_slice);
            }

            regions.insert(name.to_string(), region);

            cursor_x = cursor_x.saturating_add(img.width()).saturating_add(padding);
            row_height = row_height.max(img.height());
        }

        Self {
            image: atlas_img,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::image::Color;

    #[test]
    fn packs_row_with_padding() {
        let red = Image::new(1, 1, vec![255, 0, 0, 255], PixelFormat::Rgba8);
        let green = Image::new(
            2,
            1,
            vec![0, 255, 0, 255, 0, 255, 0, 255],
            PixelFormat::Rgba8,
        );

        let atlas = TextureAtlas::from_named(
            &[("red", &red), ("green", &green)],
            PixelFormat::Rgba8,
            6,
            2,
            1, // 1px padding
        );

        let r = atlas.region("red").unwrap();
        let g = atlas.region("green").unwrap();

        assert_eq!((r.x, r.y, r.width, r.height), (0, 0, 1, 1));
        assert_eq!((g.x, g.y, g.width, g.height), (2, 0, 2, 1)); // 1 (red) + 1 pad = 2

        let img = atlas.image();
        assert_eq!(img.get_pixel(r.x, r.y), Color::RED);
        assert_eq!(img.get_pixel(g.x, g.y), Color::GREEN);

        let uv_r = atlas.uv_rect("red").unwrap();
        let uv_g = atlas.uv_rect("green").unwrap();
        assert!(uv_r.0 < uv_r.2 && uv_r.1 < uv_r.3);
        assert!(uv_g.0 < uv_g.2 && uv_g.1 < uv_g.3);
    }

    #[test]
    #[should_panic(expected = "larger than the atlas")]
    fn rejects_image_too_large() {
        let img = Image::new(4, 1, vec![0u8; 4 * 4], PixelFormat::Rgba8);
        let _ = TextureAtlas::from_named(&[("big", &img)], PixelFormat::Rgba8, 2, 2, 0);
    }
}
