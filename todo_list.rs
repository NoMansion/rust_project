use std::collections::LinkedList;
use crate::task::Task;

pub struct ToDoList {
    pub day: String,
    pub is_complete: bool,
    pub list: LinkedList<Task>,
}

impl ToDoList {
    pub fn new(day: &str) -> Self {
        ToDoList {
            day: day.to_string(),
            is_complete: false,
            list: LinkedList::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.list.push_back(task);
    }

    pub fn remove_task(&mut self, name: &str) {
        let old_list = std::mem::take(&mut self.list);
        let mut new_list = LinkedList::new();
    
        for task in old_list {
            if task.get_name() != name {
                new_list.push_back(task);
            }
        }
    
        self.list = new_list;
    }    

    pub fn mark_all_complete(&mut self) {
        for task in &mut self.list {
            task.mark_complete();
        }
        self.is_complete = true;
    }

    pub fn mark_all_uncomplete(&mut self) {
        for task in &mut self.list {
            task.mark_uncomplete();
        }
        self.is_complete = false;
    }

    pub fn print(&self) {
        println!("To-do list {}:", self.day);
        for task in &self.list {
            task.print();
        }
    }
}
