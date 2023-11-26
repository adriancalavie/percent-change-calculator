const DEFAULT_PRECISION: usize = 4;

pub struct ProgramOptions {
    pub from: f64,
    pub to: f64,
    pub precision: usize,
}

impl Default for ProgramOptions {
    #[inline]
    fn default() -> Self {
        ProgramOptions {
            from: 0.0,
            to: 0.0,
            precision: DEFAULT_PRECISION,
        }
    }
}
