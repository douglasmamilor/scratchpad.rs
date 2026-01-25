use crate::{Point2, Vec2};

pub fn point_in_polygon(p: Point2, vertices: &[Vec2]) -> bool {
    let mut crossings = 0;

    for edge in vertices.windows(2) {
        let (a, b) = (edge[0], edge[1]);

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
