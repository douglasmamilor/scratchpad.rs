pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    // Color constants
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const RED: Color = Color {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const GREEN: Color = Color {
        r: 0,
        g: 255,
        b: 0,
        a: 255,
    };
    pub const BLUE: Color = Color {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
    };
    pub const YELLOW: Color = Color {
        r: 255,
        g: 255,
        b: 0,
        a: 255,
    };
    pub const CYAN: Color = Color {
        r: 0,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const MAGENTA: Color = Color {
        r: 255,
        g: 0,
        b: 255,
        a: 255,
    };
    pub const GRAY: Color = Color {
        r: 128,
        g: 128,
        b: 128,
        a: 255,
    };
    pub const DARK_GRAY: Color = Color {
        r: 64,
        g: 64,
        b: 64,
        a: 255,
    };
    pub const LIGHT_GRAY: Color = Color {
        r: 192,
        g: 192,
        b: 192,
        a: 255,
    };
    pub const ORANGE: Color = Color {
        r: 255,
        g: 165,
        b: 0,
        a: 255,
    };
    pub const PURPLE: Color = Color {
        r: 128,
        g: 0,
        b: 128,
        a: 255,
    };
    pub const BROWN: Color = Color {
        r: 139,
        g: 69,
        b: 19,
        a: 255,
    };
    pub const PINK: Color = Color {
        r: 255,
        g: 192,
        b: 203,
        a: 255,
    };
    pub const TRANSPARENT: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };

    #[inline]
    #[allow(non_snake_case)]
    pub fn RGB(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 255 }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn RGBA(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    #[inline]
    pub fn to_u32(&self) -> u32 {
        (self.a as u32) << 24 | (self.r as u32) << 16 | (self.g as u32) << 8 | self.b as u32
    }

    #[inline]
    pub fn from_u32(color: u32) -> Color {
        let a = ((0xff000000 & color) >> 24) as u8;
        let r = ((0x00ff0000 & color) >> 16) as u8;
        let g = ((0x0000ff00 & color) >> 8) as u8;
        let b = (0x000000ff & color) as u8;

        Color { a, r, g, b }
    }

    pub fn lerp(&self, to: &Color, tval: f32) -> Color {
        let t = tval.clamp(0.0, 1.0);

        // This uses linear interpolation (lerp) formula for each channel
        // the formula is: start + t * (end - start)
        let a = (self.a as f32 + t * (to.a as f32 - self.a as f32)).round() as u8;
        let r = (self.r as f32 + t * (to.r as f32 - self.r as f32)).round() as u8;
        let g = (self.g as f32 + t * (to.g as f32 - self.g as f32)).round() as u8;
        let b = (self.b as f32 + t * (to.b as f32 - self.b as f32)).round() as u8;

        Color { a, r, g, b }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgba_to_u32() {
        let u32_color: u32 = 0xff00ef11;
        //ARGB: (255, 0, 239, 17)

        let rgba_color_instance = Color::RGBA(0, 239, 17, 255);
        let to_u32 = rgba_color_instance.to_u32();
        assert_eq!(u32_color, to_u32);
    }

    #[test]
    fn test_u32_to_rgba() {
        let rgba_color_instance = Color::RGBA(0, 239, 17, 255);
        let u32_color: u32 = 0xff00ef11;

        let converted = Color::from_u32(u32_color);
        assert_eq!(converted.r, rgba_color_instance.r);
        assert_eq!(converted.g, rgba_color_instance.g);
        assert_eq!(converted.b, rgba_color_instance.b);
        assert_eq!(converted.a, rgba_color_instance.a);
    }

    #[test]
    fn test_lerp() {
        // Test t=0 returns start color
        let start = Color::RED;
        let end = Color::BLUE;
        let result = start.lerp(&end, 0.0);
        assert_eq!(result.r, 255);
        assert_eq!(result.g, 0);
        assert_eq!(result.b, 0);

        // Test t=1 returns end color
        let result = start.lerp(&end, 1.0);
        assert_eq!(result.r, 0);
        assert_eq!(result.g, 0);
        assert_eq!(result.b, 255);

        // Test t=0.5 returns midpoint
        let result = start.lerp(&end, 0.5);
        assert_eq!(result.r, 128); // Halfway between 255 and 0
        assert_eq!(result.g, 0);
        assert_eq!(result.b, 128); // Halfway between 0 and 255

        // Test clamping - values outside 0-1 get clamped
        let result = start.lerp(&end, -0.5);
        assert_eq!(result.r, 255); // Clamped to t=0

        let result = start.lerp(&end, 1.5);
        assert_eq!(result.b, 255); // Clamped to t=1

        // Test with custom colors including alpha
        let semi_red = Color::RGBA(255, 0, 0, 128);
        let semi_green = Color::RGBA(0, 255, 0, 128);
        let result = semi_red.lerp(&semi_green, 0.5);
        assert_eq!(result.r, 128);
        assert_eq!(result.g, 128);
        assert_eq!(result.b, 0);
        assert_eq!(result.a, 128); // Alpha stays same when both are 128
    }
}
