use crate::math::Point2;

#[derive(Debug, Default)]
pub struct PolyLine {
    points: Vec<Point2>,          // P[0..n-1]
    cumulative_lengths: Vec<f32>, // cumulative lengths at each point
    closed: bool,                 // true if ClosePath
}

impl PolyLine {
    pub fn new(points: Vec<Point2>, closed: bool) -> Self {
        let mut cumulative_lengths = Vec::with_capacity(points.len());

        assert!(points.len() >= 2, "PolyLine must have at least 2 points");

        let mut curr_len = 0.0;
        for w in points.windows(2) {
            curr_len += (w[1] - w[0]).len();
            cumulative_lengths.push(curr_len);
        }

        if closed && points.len() > 1 {
            curr_len += (points[0] - points[points.len() - 1]).len();
            cumulative_lengths.push(curr_len);
        }

        Self {
            points,
            closed,
            cumulative_lengths,
        }
    }

    /// Returns the length of the polyline
    pub fn len(&self) -> f32 {
        self.cumulative_lengths.last().copied().unwrap_or(0.0)
    }

    pub fn point_at_len(&self, s: f32) -> Point2 {
        // Invariant: polyline cannot be empty
        assert!(!self.points.is_empty(), "Polyline has no points");

        // Invariant: if only one point, return it
        if self.points.len() == 1 {
            return self.points[0];
        }

        let s = s.clamp(0.0, self.len());
        let start_i = self
            .cumulative_lengths
            .binary_search_by(|len| len.total_cmp(&s))
            .unwrap_or_else(|i| if i == 0 { 0 } else { i - 1 });

        let end_i = (start_i + 1).min(self.points.len() - 1);
        let segment_start_len = self.cumulative_lengths[start_i];
        let segment_end_len = self.cumulative_lengths[end_i];

        // If s is at or beyond the last segment, return the last point
        if s >= self.len() {
            return *self.points.last().unwrap();
        }

        let segment_len = segment_end_len - segment_start_len;
        let t_local = (s - segment_start_len) / segment_len;

        self.points[start_i].lerp(self.points[end_i], t_local)
    }
}
