use crate::math::Vec2;

pub enum Anchor {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl Anchor {
    /// Returns the offset from top-left to this anchor point
    pub fn local_offset(&self, width: f32, height: f32) -> Vec2 {
        match self {
            Anchor::TopLeft => Vec2::new(0.0, 0.0),
            Anchor::TopCenter => Vec2::new(width / 2.0, 0.0),
            Anchor::TopRight => Vec2::new(width, 0.0),
            Anchor::CenterLeft => Vec2::new(0.0, height / 2.0),
            Anchor::Center => Vec2::new(width / 2.0, height / 2.0),
            Anchor::CenterRight => Vec2::new(width, height / 2.0),
            Anchor::BottomLeft => Vec2::new(0.0, height),
            Anchor::BottomCenter => Vec2::new(width / 2.0, height),
            Anchor::BottomRight => Vec2::new(width, height),
        }
    }

    /// Returns top-left corner when this anchor is positioned at desired_point
    pub fn top_left_for(&self, point: Vec2, width: f32, height: f32) -> Vec2 {
        let offset = self.local_offset(width, height);
        point - offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_offset_top_left() {
        let anchor = Anchor::TopLeft;
        let offset = anchor.local_offset(200.0, 50.0);
        assert_eq!(offset, Vec2::new(0.0, 0.0));
    }

    #[test]
    fn local_offset_center() {
        let anchor = Anchor::Center;
        let offset = anchor.local_offset(200.0, 50.0);
        assert_eq!(offset, Vec2::new(100.0, 25.0));
    }

    #[test]
    fn local_offset_bottom_right() {
        let anchor = Anchor::BottomRight;
        let offset = anchor.local_offset(200.0, 50.0);
        assert_eq!(offset, Vec2::new(200.0, 50.0));
    }

    #[test]
    fn local_offset_top_center() {
        let anchor = Anchor::TopCenter;
        let offset = anchor.local_offset(200.0, 50.0);
        assert_eq!(offset, Vec2::new(100.0, 0.0));
    }

    #[test]
    fn local_offset_center_left() {
        let anchor = Anchor::CenterLeft;
        let offset = anchor.local_offset(200.0, 50.0);
        assert_eq!(offset, Vec2::new(0.0, 25.0));
    }

    #[test]
    fn local_offset_all_anchors() {
        let width = 100.0;
        let height = 80.0;

        assert_eq!(
            Anchor::TopLeft.local_offset(width, height),
            Vec2::new(0.0, 0.0)
        );
        assert_eq!(
            Anchor::TopCenter.local_offset(width, height),
            Vec2::new(50.0, 0.0)
        );
        assert_eq!(
            Anchor::TopRight.local_offset(width, height),
            Vec2::new(100.0, 0.0)
        );
        assert_eq!(
            Anchor::CenterLeft.local_offset(width, height),
            Vec2::new(0.0, 40.0)
        );
        assert_eq!(
            Anchor::Center.local_offset(width, height),
            Vec2::new(50.0, 40.0)
        );
        assert_eq!(
            Anchor::CenterRight.local_offset(width, height),
            Vec2::new(100.0, 40.0)
        );
        assert_eq!(
            Anchor::BottomLeft.local_offset(width, height),
            Vec2::new(0.0, 80.0)
        );
        assert_eq!(
            Anchor::BottomCenter.local_offset(width, height),
            Vec2::new(50.0, 80.0)
        );
        assert_eq!(
            Anchor::BottomRight.local_offset(width, height),
            Vec2::new(100.0, 80.0)
        );
    }

    #[test]
    fn local_offset_zero_size() {
        let anchor = Anchor::Center;
        let offset = anchor.local_offset(0.0, 0.0);
        assert_eq!(offset, Vec2::new(0.0, 0.0));
    }

    #[test]
    fn top_left_for_center() {
        let anchor = Anchor::Center;
        let centre_point = Vec2::new(640.0, 360.0);
        let top_left = anchor.top_left_for(centre_point, 200.0, 50.0);
        assert_eq!(top_left, Vec2::new(540.0, 335.0));
    }

    #[test]
    fn top_left_for_top_left() {
        let anchor = Anchor::TopLeft;
        let top_left_point = Vec2::new(100.0, 200.0);
        let top_left = anchor.top_left_for(top_left_point, 200.0, 50.0);
        assert_eq!(top_left, top_left_point); // TopLeft anchor means point IS top_left
    }

    #[test]
    fn top_left_for_bottom_right() {
        let anchor = Anchor::BottomRight;
        let bottom_right_point = Vec2::new(1000.0, 700.0);
        let top_left = anchor.top_left_for(bottom_right_point, 200.0, 50.0);
        assert_eq!(top_left, Vec2::new(800.0, 650.0));
    }

    #[test]
    fn top_left_for_top_center() {
        let anchor = Anchor::TopCenter;
        let top_center = Vec2::new(640.0, 100.0);
        let top_left = anchor.top_left_for(top_center, 200.0, 50.0);
        assert_eq!(top_left, Vec2::new(540.0, 100.0));
    }

    #[test]
    fn top_left_for_round_trip() {
        // Test that we can go from top_left -> anchor position -> top_left
        let anchor = Anchor::Center;
        let original_top_left = Vec2::new(100.0, 200.0);
        let width = 200.0;
        let height = 50.0;

        // Calculate where the center would be
        let offset = anchor.local_offset(width, height);
        let center_pos = original_top_left + offset;

        // Now calculate top_left from center
        let calculated_top_left = anchor.top_left_for(center_pos, width, height);
        assert!((calculated_top_left.x - original_top_left.x).abs() < 1e-6);
        assert!((calculated_top_left.y - original_top_left.y).abs() < 1e-6);
    }

    #[test]
    fn top_left_for_all_anchors() {
        let point = Vec2::new(500.0, 300.0);
        let width = 100.0;
        let height = 80.0;

        // TopLeft: desired_point is already top_left
        assert_eq!(Anchor::TopLeft.top_left_for(point, width, height), point);

        // TopCenter: center horizontally, top vertically
        assert_eq!(
            Anchor::TopCenter.top_left_for(point, width, height),
            Vec2::new(450.0, 300.0)
        );

        // TopRight: right edge at desired_point
        assert_eq!(
            Anchor::TopRight.top_left_for(point, width, height),
            Vec2::new(400.0, 300.0)
        );

        // CenterLeft: center vertically, left edge at desired_point
        assert_eq!(
            Anchor::CenterLeft.top_left_for(point, width, height),
            Vec2::new(500.0, 260.0)
        );

        // Center: center at desired_point
        assert_eq!(
            Anchor::Center.top_left_for(point, width, height),
            Vec2::new(450.0, 260.0)
        );

        // CenterRight: center vertically, right edge at desired_point
        assert_eq!(
            Anchor::CenterRight.top_left_for(point, width, height),
            Vec2::new(400.0, 260.0)
        );

        // BottomLeft: bottom edge at desired_point
        assert_eq!(
            Anchor::BottomLeft.top_left_for(point, width, height),
            Vec2::new(500.0, 220.0)
        );

        // BottomCenter: center horizontally, bottom edge at desired_point
        assert_eq!(
            Anchor::BottomCenter.top_left_for(point, width, height),
            Vec2::new(450.0, 220.0)
        );

        // BottomRight: bottom-right corner at desired_point
        assert_eq!(
            Anchor::BottomRight.top_left_for(point, width, height),
            Vec2::new(400.0, 220.0)
        );
    }
}
