use crate::Point2;

pub fn point_in_polygon(p: Point2, vertices: &[Point2]) -> bool {
    let mut crossings = 0;

    for i in 0..vertices.len() {
        let (a, b) = (vertices[0], vertices[(i + 1) % vertices.len()]);

        if (a.y > p.y) != (b.y > p.y) {
            let t = (p.y - a.y) / (b.y - a.y);
            let x_cross = a.x + t * (b.x - a.x);

            if x_cross > p.x {
                crossings += 1;
            }
        }
    }

    (crossings % 2) == 1
}
