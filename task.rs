// task struct for to-do lists
// created by Sam Good

// struct definition for task
pub struct Task {
    name: String,
    description: String,
    is_complete: bool,
}

impl Task {
    // constructor, takes a name and description for the task
    // automatically sets is_complete to false
    pub fn new(name: &str, description: &str) -> Self {
        Task {
            name: name.to_string(),
            description: description.to_string(),
            is_complete: false,
        }
    }

    // marks a task as complete
    pub fn mark_complete(&mut self) {
        self.is_complete = true;
    }

    // marks a task as incomplete
    pub fn mark_incomplete(&mut self) {
        self.is_complete = false;
    }

    // prints the name, desciption, and status of a task
    pub fn print(&self) {
        println!(
            "{}: {} [{}]",
            self.name,
            self.description,
            if self.is_complete { "Done" } else { "Not Done" }
        );
    }

    // gets the name of the task
    pub fn get_name(&self) -> &str {
        &self.name
    }
}
