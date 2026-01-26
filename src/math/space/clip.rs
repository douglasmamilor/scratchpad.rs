use crate::{Point2, Rect};

#[derive(Copy, Clone, Debug)]
pub enum Edge {
    Left,
    Right,
    Bottom,
    Top,
}

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

fn inside(p: Point2, rect: Rect, edge: Edge) -> bool {
    match edge {
        Edge::Left => p.x >= rect.x,
        Edge::Right => p.x <= rect.x + rect.width,
        Edge::Bottom => p.y >= rect.y,
        Edge::Top => p.y <= rect.y + rect.height,
    }
}

fn intersect(s: Point2, e: Point2, rect: Rect, edge: Edge) -> Point2 {
    let dx = e.x - s.x;
    let dy = e.y - s.y;
    let x_min = rect.x;
    let x_max = rect.x + rect.width;
    let y_min = rect.y;
    let y_max = rect.y + rect.height;

    match edge {
        Edge::Left => {
            let t = (x_min - s.x) / dx;
            Point2 {
                x: x_min,
                y: s.y + t * dy,
            }
        }

        Edge::Right => {
            let t = (x_max - s.x) / dx;
            Point2 {
                x: x_max,
                y: s.y + t * dy,
            }
        }

        Edge::Bottom => {
            let t = (y_min - s.y) / dy;
            Point2 {
                x: s.x + t * dx,
                y: y_min,
            }
        }

        Edge::Top => {
            let t = (y_max - s.y) / dy;
            Point2 {
                x: s.x + t * dx,
                y: y_max,
            }
        }
    }
}

fn clip_against_edge(input: &[Point2], rect: Rect, edge: Edge) -> Vec<Point2> {
    let mut output = Vec::new();

    if input.is_empty() {
        return output;
    }

    let mut s = input[input.len() - 1];

    for &e in input {
        let s_in = inside(s, rect, edge);
        let e_in = inside(e, rect, edge);

        match (s_in, e_in) {
            // Case 1: inside → inside
            (true, true) => {
                output.push(e);
            }

            // Case 2: inside → outside
            (true, false) => {
                let i = intersect(s, e, rect, edge);
                output.push(i);
            }

            // Case 3: outside → inside
            (false, true) => {
                let i = intersect(s, e, rect, edge);
                output.push(i);
                output.push(e);
            }

            // Case 4: outside → outside
            (false, false) => {
                // emit nothing
            }
        }

        s = e;
    }

    output
}

pub fn clip_polygon(poly: &[Point2], rect: Rect) -> Vec<Point2> {
    let poly = clip_against_edge(poly, rect, Edge::Left);
    let poly = clip_against_edge(&poly, rect, Edge::Right);
    let poly = clip_against_edge(&poly, rect, Edge::Bottom);

    clip_against_edge(&poly, rect, Edge::Top)
}
