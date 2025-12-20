#[derive(Debug, Default)]
pub struct Polyline {
    points: Vec<Point2>, // P[0..n-1]
    closed: bool,        // true if ClosePath
}

impl PolyLine {
    pub fn len(&self) -> f32 {
        let mut lenght = 0.0;

        for w in self.points.windwos(2) {}
    }
}
