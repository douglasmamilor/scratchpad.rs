use crate::color::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineCap {
    Butt,
    Square,
    Round,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineJoin {
    Miter { limit: f32 },
    Bevel,
    Round,
}

pub enum StrokeSpace {
    ScreenSpace { thickness: u64 },
    WorldSpace { thickness: u64 },
}

pub enum StrokePattern {
    Dashed {
        dash_length: f32,
        gap_length: f32,
        phase: f32,
        enabled: bool,
    },
    Dotted {
        dot_space: f32,
        dot_radius: f32,
        phase: f32,
        enabled: bool,
    },
}

pub struct StrokeStyle {
    space: StrokeSpace,
    pattern: StrokePattern,
    cap: LineCap,
    join: LineJoin,
    color: Color,
}

impl StrokeStyle {
    /// Minimal “solid stroke” style (no pattern).
    pub fn solid_screen_px(thickness_px: f32, color: Color) -> Self {
        StrokeStyle {
            space: StrokeSpace::ScreenSpace {
                thickness: thickness_px.max(0.0) as u64,
            },
            pattern: StrokePattern::Dashed {
                dash_length: 1.0,
                gap_length: 0.0,
                phase: 0.0,
                enabled: false,
            },
            cap: LineCap::Butt,
            join: LineJoin::Bevel,
            color,
        }
    }

    /// Builder-ish setters (return updated copies).
    pub fn with_cap(mut self, cap: LineCap) -> Self {
        self.cap = cap;
        self
    }

    pub fn with_join(mut self, join: LineJoin) -> Self {
        self.join = join;
        self
    }

    pub fn with_pattern(mut self, pattern: StrokePattern) -> Self {
        self.pattern = pattern;
        self
    }

    pub fn with_space(mut self, space: StrokeSpace) -> Self {
        self.space = space;
        self
    }

    // Accessors (useful outside the module while fields stay private)
    pub fn cap(&self) -> LineCap {
        self.cap
    }
    pub fn join(&self) -> LineJoin {
        self.join
    }
    pub fn color(&self) -> Color {
        self.color
    }
    pub fn space(&self) -> &StrokeSpace {
        &self.space
    }
    pub fn pattern(&self) -> &StrokePattern {
        &self.pattern
    }
}
