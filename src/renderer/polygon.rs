use core::f32;

use crate::{Point2, Vec2};

use super::Renderer;
use crate::Color;
use crate::math::Mat3;
use crate::math::space::clip::clip_polygon;

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    y_max: i32,
    x: f32,      // x at current scanline starting from y_min
    winding: i8, // +1 or -1 for non-zero winding
    dx_dy: f32,  // inverse slope
}

pub enum FillRule {
    EvenOdd,
    NonZeroWinding,
}

type EdgeTable = std::collections::HashMap<i32, Vec<Edge>>;
pub struct EdgeTableResult {
    edge_table: EdgeTable,
    y_min: i32,
    y_max: i32,
}

impl<'a> Renderer<'a> {
    /// Draw a polygon outline by connecting vertices with lines
    ///
    /// # Arguments
    /// * `vertices` - Array of Vec2 points defining the polygon
    /// * `color` - Color for the outline
    ///
    /// # Example
    /// ```
    /// use scratchpad_rs::color::Color;
    /// use scratchpad_rs::math::{vec2::Vec2, Mat3};
    /// use scratchpad_rs::renderer::Renderer;
    /// use scratchpad_rs::framebuffer::FrameBuffer;
    ///
    /// let mut framebuffer = FrameBuffer::new(800, 600);
    /// let mut renderer = Renderer::new(&mut framebuffer);
    ///
    /// let vertices = vec![
    ///     Vec2::new(100.0, 100.0),  // Top
    ///     Vec2::new(150.0, 150.0),  // Right
    ///     Vec2::new(100.0, 200.0),  // Bottom
    ///     Vec2::new(50.0, 150.0),   // Left
    /// ];
    /// renderer.draw_polygon(&vertices, Color::RED, Mat3::IDENTITY);
    /// ```
    pub fn draw_polygon(&mut self, vertices: &[Vec2], color: Color, model: Mat3) {
        if vertices.len() < 2 {
            return;
        }

        // walk each consecutive pair
        for w in vertices.windows(2) {
            self.draw_line_aa(w[0], w[1], color, model);
        }

        // close polygon
        let last = vertices.last().unwrap();
        let first = vertices.first().unwrap();
        self.draw_line_aa(*last, *first, color, model);
    }

    pub fn draw_regular_polygon(
        &mut self,
        ctr: Vec2,
        r: f32,
        rot: f32,
        sides: usize,
        color: Color,
        model: Mat3,
    ) {
        let step = std::f32::consts::TAU / sides as f32;
        let mut vertices: Vec<Vec2> = Vec::with_capacity(sides);

        for i in 0..sides {
            let angle = rot + i as f32 * step;
            let x = ctr.x + r * angle.cos();
            let y = ctr.y + r * angle.sin();

            vertices.push(Vec2::from((x, y)))
        }

        self.draw_polygon(&vertices, color, model);
    }

    pub fn fill_polygon(&mut self, vertices: Vec<Point2>, color: Color, fill_rule: FillRule) {
        let mut vertices = vertices;
        if vertices.len() < 3 {
            return;
        }

        // Clip to the active viewport/scissor rect, if any.
        if let Some(clip_rect) = self.active_clip_rect() {
            let clipped = clip_polygon(&vertices, clip_rect);
            if clipped.len() < 3 {
                return;
            }
            vertices = clipped;
        }

        if let Some(etr) = self.build_edge_table(vertices) {
            self.scan_convert(color, fill_rule, etr);
        }
    }

    pub fn build_edge_table(&mut self, vertices: Vec<Point2>) -> Option<EdgeTableResult> {
        let mut edge_table: EdgeTable = EdgeTable::new();
        let mut ymin_global = f32::INFINITY;
        let mut ymax_global = f32::NEG_INFINITY;

        let v_len = vertices.len();
        for i in 0..v_len {
            let (v0, v1) = (vertices[i], vertices[(i + 1) % v_len]);

            let (x0, x1) = (v0.x, v1.x);
            let (y0, y1) = (v0.y, v1.y);

            let dy = y1 - y0;
            // Degenerate case: horizontal line
            if dy.abs() < f32::EPSILON {
                continue;
            }

            let dx_dy = (x1 - x0) / dy;

            let (y_min, y_max, x_at_ymin, winding) = if y0 < y1 {
                let y_min = y0.floor();
                let y_max = y1.ceil();
                let x_at_ymin = x0 + (y_min - y0) * dx_dy;
                (y_min, y_max, x_at_ymin, 1)
            } else {
                let y_min = y1.floor();
                let y_max = y0.ceil();
                let x_at_ymin = x1 + (y_min - y1) * dx_dy;
                (y_min, y_max, x_at_ymin, -1)
            };

            edge_table.entry(y_min as i32).or_default().push(Edge {
                y_max: y_max as i32,
                x: x_at_ymin,
                winding,
                dx_dy,
            });

            ymin_global = ymin_global.min(y_min);
            ymax_global = ymax_global.max(y_max);
        }

        Some(EdgeTableResult {
            edge_table,
            y_min: ymin_global as i32,
            y_max: ymax_global as i32,
        })
    }

    pub fn scan_convert(&mut self, color: Color, fill_rule: FillRule, etr: EdgeTableResult) {
        let y_min = etr.y_min;
        let y_max = etr.y_max;
        let edge_table = etr.edge_table;
        let mut active_edge_table: Vec<Edge> = Vec::new();

        for scan_line in y_min..y_max {
            if let Some(edges_starting_here) = edge_table.get(&scan_line) {
                active_edge_table.extend(edges_starting_here.iter().cloned());
            }

            // Keep edges whose y_max is above the scanline.
            // Drop those whose y_max is at or below the scanline
            active_edge_table.retain(|e| e.y_max > scan_line);

            // Sort current x
            active_edge_table.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

            match fill_rule {
                FillRule::EvenOdd => {
                    let mut inside = false;
                    let mut prev_x: f32 = 0.0;
                    for edge in &active_edge_table {
                        if !inside {
                            prev_x = edge.x;
                            inside = true;
                        } else {
                            let (x0, x1) = (prev_x.floor(), edge.x.ceil());
                            self.hspan(scan_line, x0 as i32, x1 as i32, color);
                            inside = false;
                        }
                    }
                }
                FillRule::NonZeroWinding => {
                    let mut winding_count: i8 = 0;
                    let mut prev_x: f32 = 0.0;
                    for edge in &active_edge_table {
                        // If we are outside, then we are crossing an edge
                        if winding_count == 0 {
                            prev_x = edge.x;
                        }
                        winding_count += edge.winding;
                        // At the next iteration we wound up outside again,
                        // so we crossed another edge
                        if winding_count == 0 {
                            let (x0, x1) = (prev_x.floor(), edge.x.ceil());
                            self.hspan(scan_line, x0 as i32, x1 as i32, color);
                        }
                    }
                }
            }
            // Increment x for all edges in AET
            for e in &mut active_edge_table {
                e.x += e.dx_dy;
            }
        }
    }
}
