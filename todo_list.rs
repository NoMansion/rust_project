// To-do list struct
// Created by Sam Good

use crate::task::Task;
use std::collections::LinkedList;

// struct definition
pub struct ToDoList {
    pub title: String,
    pub is_complete: bool,
    pub list: LinkedList<Task>,
}

impl ToDoList {
    // constructor, takes a name for the to-do list
    pub fn new(title: &str) -> Self {
        ToDoList {
            title: title.to_string(),
            is_complete: false,
            list: LinkedList::new(),
        }
    }

    // adds a task to the to-do list
    pub fn add_task(&mut self, task: Task) {
        self.list.push_back(task);
    }

    // removes a task from the to-do list
    // takes the name of the task
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

    // marks all tasks as complete
    pub fn mark_all_complete(&mut self) {
        for task in &mut self.list {
            task.mark_complete();
        }
        self.is_complete = true;
    }

    // marks all tasks as incomplete
    pub fn mark_all_incomplete(&mut self) {
        for task in &mut self.list {
            task.mark_incomplete();
        }
        self.is_complete = false;
    }

    // prints the title and all tasks with their desciptions
    pub fn print(&self) {
        println!("To-do list {}:", self.title);
        for task in &self.list {
            task.print();
        }
    }

    // checks if a to-do list is empty
    pub fn check_empty(&self) -> bool {
        self.list.is_empty()
    }
}
