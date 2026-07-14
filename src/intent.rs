//! Intent module - Human input layer

pub struct Intent {
    pub goal: String,
    pub constraints: Vec<String>,
}

impl Intent {
    pub fn new(goal: String) -> Self {
        Self {
            goal,
            constraints: Vec::new(),
        }
    }
}