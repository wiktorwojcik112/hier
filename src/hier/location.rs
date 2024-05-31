#[derive(Debug, Clone)]
pub struct Location {
    pub module: String,
    pub line_number: i64,
    pub offset: i64
}

impl Location {
    pub fn new(module: String, line_number: i64, offset: i64) -> Self {
        Self {
            module,
            line_number,
            offset
        }
    }

    pub fn empty() -> Self {
        Self {
            module: String::new(),
            line_number: 0,
            offset: 0
        }
    }
}