pub struct Condition {
    description: String,
}

impl Condition {
    pub fn new(description: &str) -> Self {
        Condition {
            description: String::from(description),
        }
    }
}
