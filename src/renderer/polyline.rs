use crate::math::Point2;

#[derive(Debug, Default, Clone)]
pub struct PolyLine {
    points: Vec<Point2>,          // P[0..n-1]
    cumulative_lengths: Vec<f32>, // length at each point index (starts with 0)
    closed: bool,                 // true if ClosePath
}

impl PolyLine {
    pub fn new(points: Vec<Point2>, closed: bool) -> Self {
        assert!(points.len() >= 2, "PolyLine must have at least 2 points");

        // cumulative_lengths[i] = arc-length at points[i]
        let mut cumulative_lengths = Vec::with_capacity(points.len() + if closed { 1 } else { 0 });
        cumulative_lengths.push(0.0);

        let mut curr_len = 0.0;
        for w in points.windows(2) {
            curr_len += (w[1] - w[0]).len();
            cumulative_lengths.push(curr_len);
        }

        // If closed, store one extra length entry for the closing edge
        // (points are still P[0..n-1], but len() includes the closing segment)
        if closed {
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

    pub fn points(&self) -> &[Point2] {
        &self.points
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }

    /// Point along the polyline at arc-length `s` (clamped to [0, len]).
    pub fn point_at_len(&self, s: f32) -> Point2 {
        assert!(!self.points.is_empty(), "Polyline has no points");

        if self.points.len() == 1 {
            return self.points[0];
        }

        let total = self.len();
        let s = s.clamp(0.0, total);

        if s >= total {
            // If closed, you could return points[0], but for dashing it’s fine to return last.
            return *self.points.last().unwrap();
        }

        // For OPEN: cumulative_lengths has length points.len()
        // For CLOSED: cumulative_lengths has length points.len()+1, last entry includes close edge
        // For sampling along the “explicit points chain”, we treat segments among points first.
        // If closed and s goes into the closing edge, we interpolate last->first.
        let n = self.points.len();

        // Find segment i such that cum[i] <= s <= cum[i+1] (among the explicit chain)
        // cum entries for explicit points are indices [0..n-1] inclusive => length n
        let chain_end = n; // number of cum entries that correspond to point indices (0..n-1)
        let cum_chain = &self.cumulative_lengths[..chain_end];

        match cum_chain.binary_search_by(|v| v.total_cmp(&s)) {
            Ok(i) => {
                // exactly at a vertex
                return self.points[i.min(n - 1)];
            }
            Err(i) => {
                // s is between cum[i-1] and cum[i]
                let seg_i = i.saturating_sub(1);

                // If seg_i is the last segment in the chain (between n-2 and n-1)
                // we're still fine for open polylines.
                if seg_i < n - 1 {
                    let a = cum_chain[seg_i];
                    let b = cum_chain[seg_i + 1];
                    let t = if b > a { (s - a) / (b - a) } else { 0.0 };
                    return self.points[seg_i].lerp(self.points[seg_i + 1], t);
                }

                // If we got here, it means s is beyond the chain (only possible if closed
                // and s is on the closing edge).
                if self.closed {
                    let a = self.cumulative_lengths[n - 1]; // length at last point
                    let b = *self.cumulative_lengths.last().unwrap(); // total incl close edge
                    let t = if b > a { (s - a) / (b - a) } else { 0.0 };
                    return self.points[n - 1].lerp(self.points[0], t);
                }

                *self.points.last().unwrap()
            }
        }
    }

    /// Extract a sub-polyline from arc-length [s0, s1] (open).
    /// Returns at least 2 points when s1 > s0.
    pub fn slice_by_len(&self, s0: f32, s1: f32) -> Option<PolyLine> {
        let total = self.len();
        let a = s0.clamp(0.0, total);
        let b = s1.clamp(0.0, total);
        if b <= a {
            return None;
        }

        let mut out: Vec<Point2> = Vec::new();
        out.push(self.point_at_len(a));

        // Add any interior vertices that lie strictly inside (a,b)
        let n = self.points.len();
        // explicit vertex lengths are indices [0..n-1]
        for i in 1..n - 1 {
            let li = self.cumulative_lengths[i];
            if li > a && li < b {
                out.push(self.points[i]);
            }
        }

        out.push(self.point_at_len(b));

        // Dedup accidental duplicates (e.g., a at a vertex)
        out.dedup_by(|p, q| (*p - *q).len() < 1e-6);

        if out.len() >= 2 {
            Some(PolyLine::new(out, false))
        } else {
            None
        }
    }
}
