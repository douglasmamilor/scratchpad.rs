#[derive(Debug, Clone)]
pub enum DepthFunc {
    Less,
    LessEq,
}

#[derive(Debug, Clone)]
pub struct DepthState {
    pub enabled: bool,
    pub write_enabled: bool,
    pub func: DepthFunc,
    pub clear_value: f32,
}

impl Default for DepthState {
    fn default() -> Self {
        Self {
            enabled: true,
            write_enabled: true,
            func: DepthFunc::Less,
            clear_value: 1.0, // far
        }
    }
}
