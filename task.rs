// src/task.rs
pub struct Task {
    name: String,
    description: String,
    is_complete: bool,
}

impl Task {
    pub fn new(name: &str, description: &str) -> Self {
        Task {
            name: name.to_string(),
            description: description.to_string(),
            is_complete: false,
        }
    }

    pub fn mark_complete(&mut self) {
        self.is_complete = true;
    }

    pub fn mark_uncomplete(&mut self) {
        self.is_complete = false;
    }
    pub fn print(&self) {
        println!(
            "{}: {} [{}]",
            self.name,
            self.description,
            if self.is_complete { "Done" } else { "Not Done" }
        );
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
    
}
