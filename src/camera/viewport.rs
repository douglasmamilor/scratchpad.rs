use super::Camera;
use crate::math::Rect;

impl Camera {
    /// Calculate viewport with letterboxing (black bars top/bottom)
    /// 
    /// Used when framebuffer is taller than target aspect ratio.
    /// Uses full width, calculates height to match target aspect, centers vertically.
    pub fn calculate_letterbox_viewport(
        &self,
        target_aspect_ratio: f32,
        framebuffer_width: usize,
        framebuffer_height: usize,
    ) -> Rect {
        let viewport_width = framebuffer_width as f32;
        let viewport_height = viewport_width / target_aspect_ratio;
        let viewport_x = 0.0;
        let viewport_y = (framebuffer_height as f32 - viewport_height) / 2.0;

        Rect {
            x: viewport_x,
            y: viewport_y,
            width: viewport_width,
            height: viewport_height,
        }
    }

    /// Calculate viewport with pillarboxing (black bars left/right)
    /// 
    /// Used when framebuffer is wider than target aspect ratio.
    /// Uses full height, calculates width to match target aspect, centers horizontally.
    pub fn calculate_pillarbox_viewport(
        &self,
        target_aspect_ratio: f32,
        framebuffer_width: usize,
        framebuffer_height: usize,
    ) -> Rect {
        let viewport_height = framebuffer_height as f32;
        let viewport_width = viewport_height * target_aspect_ratio;
        let viewport_x = (framebuffer_width as f32 - viewport_width) / 2.0;
        let viewport_y = 0.0;

        Rect {
            x: viewport_x,
            y: viewport_y,
            width: viewport_width,
            height: viewport_height,
        }
    }

    /// Calculate viewport that maintains target aspect ratio
    /// 
    /// Automatically chooses letterboxing or pillarboxing based on framebuffer dimensions.
    /// This is the recommended function for most use cases.
    pub fn calculate_fitted_viewport(
        &self,
        target_aspect_ratio: f32,
        framebuffer_width: usize,
        framebuffer_height: usize,
    ) -> Rect {
        let framebuffer_aspect = framebuffer_width as f32 / framebuffer_height as f32;

        if framebuffer_aspect > target_aspect_ratio {
            // Framebuffer is wider = need pillarboxing (bars on sides)
            self.calculate_pillarbox_viewport(target_aspect_ratio, framebuffer_width, framebuffer_height)
        } else {
            // Framebuffer is taller or equal = need letterboxing (bars top/bottom)
            self.calculate_letterbox_viewport(target_aspect_ratio, framebuffer_width, framebuffer_height)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::camera::Camera;
    use crate::math::{Point2, Rect};

    fn create_test_camera() -> Camera {
        Camera::new(Point2::ZERO, 1.0, 0.0, Rect::new(0.0, 0.0, 800.0, 600.0))
    }


    #[test]
    fn letterbox_taller_framebuffer() {
        // Framebuffer: 1920x1200 (1.6 aspect)
        // Target: 16:9 (1.777...)
        // Framebuffer is TALLER, so letterbox uses full width, calculates height
        let camera = create_test_camera();
        let target_aspect = 16.0 / 9.0; // 1.777...
        let viewport = camera.calculate_letterbox_viewport(target_aspect, 1920, 1200);

        // Should use full width (1920)
        assert_eq!(viewport.width, 1920.0);
        // Height = 1920 / 1.777... = 1080
        assert!((viewport.height - 1080.0).abs() < 1e-5);
        // Centered vertically: (1200 - 1080) / 2 = 60
        assert_eq!(viewport.y, 60.0);
        assert_eq!(viewport.x, 0.0);
    }

    #[test]
    fn pillarbox_wider_framebuffer() {
        // Framebuffer: 1920x1080 (16:9 = 1.777...)
        // Target: 4:3 (1.333...)
        // Framebuffer is WIDER, so pillarbox uses full height, calculates width
        let camera = create_test_camera();
        let target_aspect = 4.0 / 3.0; // 1.333...
        let viewport = camera.calculate_pillarbox_viewport(target_aspect, 1920, 1080);

        // Should use full height (1080)
        assert_eq!(viewport.height, 1080.0);
        // Width = 1080 * 1.333... = 1440
        assert!((viewport.width - 1440.0).abs() < 1e-5);
        // Centered horizontally: (1920 - 1440) / 2 = 240
        assert_eq!(viewport.x, 240.0);
        assert_eq!(viewport.y, 0.0);
    }

    #[test]
    fn pillarbox_taller_framebuffer() {
        // Framebuffer: 1440x1080 (4:3 = 1.333...)
        // Target: 16:9 (1.777...)
        // Framebuffer is TALLER, so pillarbox uses full height, calculates width
        // (But this would make it wider than framebuffer - edge case)
        let camera = create_test_camera();
        let target_aspect = 16.0 / 9.0; // 1.777...
        let viewport = camera.calculate_pillarbox_viewport(target_aspect, 1440, 1080);

        // Should use full height (1080)
        assert_eq!(viewport.height, 1080.0);
        // Width = 1080 * 1.777... = 1920 (but framebuffer is only 1440 wide!)
        assert!((viewport.width - 1920.0).abs() < 1e-5);
        // Centered horizontally: (1440 - 1920) / 2 = -240 (negative!)
        assert_eq!(viewport.x, -240.0);
        assert_eq!(viewport.y, 0.0);
        
        // Note: This shows why fitted_viewport is important - it chooses the right function
    }

    #[test]
    fn fitted_viewport_chooses_letterbox() {
        // Framebuffer: 1920x1200 (1.6) - TALLER
        // Target: 16:9 (1.777...) - WIDER
        // Should choose letterboxing
        let camera = create_test_camera();
        let target_aspect = 16.0 / 9.0;
        let viewport = camera.calculate_fitted_viewport(target_aspect, 1920, 1200);

        // Should use letterbox (full width, calculated height)
        assert_eq!(viewport.width, 1920.0);
        assert!((viewport.height - 1080.0).abs() < 1e-5);
        assert_eq!(viewport.x, 0.0);
        assert_eq!(viewport.y, 60.0); // Centered vertically
    }

    #[test]
    fn fitted_viewport_chooses_pillarbox() {
        // Framebuffer: 1920x1080 (1.777...) - WIDER
        // Target: 4:3 (1.333...) - NARROWER
        // Should choose pillarboxing
        let camera = create_test_camera();
        let target_aspect = 4.0 / 3.0;
        let viewport = camera.calculate_fitted_viewport(target_aspect, 1920, 1080);

        // Should use pillarbox (full height, calculated width)
        assert_eq!(viewport.height, 1080.0);
        assert!((viewport.width - 1440.0).abs() < 1e-5);
        assert_eq!(viewport.x, 240.0); // Centered horizontally
        assert_eq!(viewport.y, 0.0);
    }

    #[test]
    fn fitted_viewport_exact_match() {
        // Framebuffer: 1920x1080 (16:9)
        // Target: 16:9
        // Exact match - no bars needed
        let camera = create_test_camera();
        let target_aspect = 16.0 / 9.0;
        let viewport = camera.calculate_fitted_viewport(target_aspect, 1920, 1080);

        // Should fill entire framebuffer
        assert_eq!(viewport.width, 1920.0);
        assert_eq!(viewport.height, 1080.0);
        assert_eq!(viewport.x, 0.0);
        assert_eq!(viewport.y, 0.0);
    }

    #[test]
    fn letterbox_exact_square() {
        // Framebuffer: 1920x1080 (16:9)
        // Target: 1:1 (square)
        let camera = create_test_camera();
        let target_aspect = 1.0;
        let viewport = camera.calculate_letterbox_viewport(target_aspect, 1920, 1080);

        // Width = 1920, height = 1920 / 1.0 = 1920
        assert_eq!(viewport.width, 1920.0);
        assert_eq!(viewport.height, 1920.0);
        // But framebuffer is only 1080 tall, so y would be negative
        // This shows letterbox assumes framebuffer is tall enough
        assert_eq!(viewport.y, (1080.0 - 1920.0) / 2.0);
    }

    #[test]
    fn pillarbox_exact_square() {
        // Framebuffer: 1080x1920 (portrait, 0.5625)
        // Target: 1:1 (square)
        let camera = create_test_camera();
        let target_aspect = 1.0;
        let viewport = camera.calculate_pillarbox_viewport(target_aspect, 1080, 1920);

        // Height = 1920, width = 1920 * 1.0 = 1920
        assert_eq!(viewport.height, 1920.0);
        assert_eq!(viewport.width, 1920.0);
        // Centered: (1080 - 1920) / 2 = -420
        assert_eq!(viewport.x, (1080.0 - 1920.0) / 2.0);
        assert_eq!(viewport.y, 0.0);
    }

    #[test]
    fn fitted_viewport_common_ratios() {
        let camera = create_test_camera();

        // Test 16:9 on various framebuffer sizes
        let aspect_16_9 = 16.0 / 9.0;
        
        // Standard HD
        let v1 = camera.calculate_fitted_viewport(aspect_16_9, 1920, 1080);
        assert_eq!(v1.width, 1920.0);
        assert_eq!(v1.height, 1080.0);

        // Wider (ultrawide)
        let v2 = camera.calculate_fitted_viewport(aspect_16_9, 2560, 1080);
        // Should use pillarbox (framebuffer wider than target)
        assert_eq!(v2.height, 1080.0);
        assert!((v2.width - 1920.0).abs() < 1e-5);
        assert_eq!(v2.x, 320.0); // (2560 - 1920) / 2

        // Taller
        let v3 = camera.calculate_fitted_viewport(aspect_16_9, 1920, 1200);
        // Should use letterbox (framebuffer taller than target)
        assert_eq!(v3.width, 1920.0);
        assert!((v3.height - 1080.0).abs() < 1e-5);
        assert_eq!(v3.y, 60.0); // (1200 - 1080) / 2
    }

    #[test]
    fn viewport_aspect_ratio_preserved() {
        let camera = create_test_camera();
        let target_aspect = 16.0 / 9.0;

        // Test that calculated viewport actually matches target aspect
        let viewports = vec![
            camera.calculate_fitted_viewport(target_aspect, 1920, 1080),
            camera.calculate_fitted_viewport(target_aspect, 2560, 1440),
            camera.calculate_fitted_viewport(target_aspect, 1280, 800),
        ];

        for viewport in viewports {
            let calculated_aspect = viewport.width / viewport.height;
            assert!(
                (calculated_aspect - target_aspect).abs() < 1e-5,
                "Viewport aspect {} should match target {}",
                calculated_aspect,
                target_aspect
            );
        }
    }

    #[test]
    fn letterbox_common_cases() {
        let camera = create_test_camera();

        // Standard letterbox case: 16:9 content on 16:10 display
        let v = camera.calculate_letterbox_viewport(16.0 / 9.0, 1920, 1200);
        assert_eq!(v.width, 1920.0);
        assert!((v.height - 1080.0).abs() < 1e-5);
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 60.0);
    }

    #[test]
    fn pillarbox_common_cases() {
        let camera = create_test_camera();

        // Standard pillarbox case: 4:3 content on 16:9 display
        let v = camera.calculate_pillarbox_viewport(4.0 / 3.0, 1920, 1080);
        assert_eq!(v.height, 1080.0);
        assert!((v.width - 1440.0).abs() < 1e-5);
        assert_eq!(v.x, 240.0);
        assert_eq!(v.y, 0.0);
    }
}
