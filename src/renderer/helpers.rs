use crate::math::vec2::Vec2;

/// Quantizes a floating-point point to integer pixel coordinates.
///
/// Uses rounding to ensure the closest pixel is selected.
///
/// # Examples
/// ```text
/// use scratchpad_rs::math::Vec2;
/// use scratchpad_rs::renderer::quantize_point;
///
/// let point = Vec2::new(10.7, 15.3);
/// let (x, y) = quantize_point(point);
/// assert_eq!((x, y), (11, 15));
/// ```
#[inline]
pub(crate) fn quantize_point(p: Vec2) -> (i32, i32) {
    (p.x.round() as i32, p.y.round() as i32)
}

/// Quantizes a horizontal span to integer pixel coordinates.
///
/// Uses half-open interval [x0, x1) to avoid off-by-one errors.
///
/// # Examples
/// ```text
/// use scratchpad_rs::renderer::quantize_hspan;
///
/// let (y, x0, x1) = quantize_hspan(10.7, 5.2, 15.8);
/// assert_eq!((y, x0, x1), (11, 5, 16));
/// ```
#[inline]
pub(crate) fn quantize_hspan(y: f32, x0: f32, x1: f32) -> (i32, i32, i32) {
    // half-open [x0, x1), avoids off-by-one
    (y.round() as i32, x0.floor() as i32, x1.ceil() as i32)
}

/// Quantizes a vertical span to integer pixel coordinates.
///
/// Uses half-open interval [y0, y1) to avoid off-by-one errors.
///
/// # Examples
/// ```text
/// use scratchpad_rs::renderer::quantize_vspan;
///
/// let (x, y0, y1) = quantize_vspan(10.7, 5.2, 15.8);
/// assert_eq!((x, y0, y1), (11, 5, 16));
/// ```
#[inline]
pub(crate) fn quantize_vspan(x: f32, y0: f32, y1: f32) -> (i32, i32, i32) {
    (x.round() as i32, y0.floor() as i32, y1.ceil() as i32)
}

/// Snaps a coordinate to pixel boundaries for crisp 1-pixel lines.
///
/// Handles half-pixel offsets to ensure lines appear crisp at any position.
///
/// # Examples
/// ```text
/// use scratchpad_rs::renderer::snap_axis;
///
/// let snapped = snap_axis(10.3, 1.0); // 1-pixel line
/// assert_eq!(snapped, 10.5); // Snapped to half-pixel for crisp line
///
/// let snapped = snap_axis(10.3, 2.0); // 2-pixel line
/// assert_eq!(snapped, 10.0); // Snapped to pixel boundary
/// ```
#[inline]
pub(crate) fn snap_axis(v: f32, stroke_px: f32) -> f32 {
    let odd = (stroke_px.round() as i32) & 1 == 1;
    if odd { v.round() + 0.5 } else { v.round() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Vec2;

    #[test]
    fn quantize_point_rounds() {
        assert_eq!(quantize_point(Vec2::new(10.7, 15.3)), (11, 15));
        assert_eq!(quantize_point(Vec2::new(10.2, 15.8)), (10, 16));
    }

    #[test]
    fn quantize_hspan_half_open() {
        assert_eq!(quantize_hspan(10.7, 5.2, 15.8), (11, 5, 16));
        assert_eq!(quantize_hspan(10.3, 5.0, 5.0), (10, 5, 5));
    }

    #[test]
    fn quantize_vspan_half_open() {
        assert_eq!(quantize_vspan(10.7, 5.2, 15.8), (11, 5, 16));
        assert_eq!(quantize_vspan(10.3, 5.0, 5.0), (10, 5, 5));
    }

    #[test]
    fn snap_axis_aligns_odd_and_even_widths() {
        assert_eq!(snap_axis(10.3, 1.0), 10.5);
        assert_eq!(snap_axis(10.3, 2.0), 10.0);
        assert_eq!(snap_axis(10.7, 3.0), 11.5);
    }
}
