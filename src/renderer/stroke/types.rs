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

/// Which space pattern lengths/radii are defined in.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PatternSpace {
    /// Pattern lengths are in the same space as stroking (screen pixels for ScreenSpace strokes;
    /// world units for WorldSpace strokes). This keeps dash/dot lengths stable under transforms.
    StrokeSpace,
    /// Pattern lengths are in the original path space. Model transforms will scale the pattern.
    PathSpace,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StrokeSpace {
    ScreenSpace { thickness: u64 },
    WorldSpace { thickness: u64 },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StrokePattern {
    Dashed {
        dash_length: f32,
        gap_length: f32,
        phase: f32,
        enabled: bool,
        space: PatternSpace,
    },
    Dotted {
        dot_space: f32,
        dot_radius: f32,
        phase: f32,
        enabled: bool,
        space: PatternSpace,
    },
}

#[derive(Debug, Clone)]
pub struct StrokeStyle {
    space: StrokeSpace,
    pattern: StrokePattern,
    cap: LineCap,
    join: LineJoin,
    color: Color,
    curve_tolerance: f32,
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
                space: PatternSpace::StrokeSpace,
            },
            cap: LineCap::Butt,
            join: LineJoin::Bevel,
            color,
            curve_tolerance: 0.5,
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

    pub fn with_curve_tolerance(mut self, tolerance: f32) -> Self {
        self.curve_tolerance = tolerance;
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
    pub fn curve_tolerance(&self) -> f32 {
        self.curve_tolerance
    }
}
