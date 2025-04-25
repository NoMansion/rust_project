// Program to create and modify to-do lists
// Created by Sam Good

mod task;
mod todo_list;

use std::collections::LinkedList;
use std::io::{self, Write};
use task::Task;
use todo_list::ToDoList;

// method to ask the user a question.
fn input(prompt: &str) -> String {
    print!("{}", prompt);
    let _ = io::stdout().flush();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

// method to find a to-do list by searching for its name
fn find_list_mut<'a>(lists: &'a mut LinkedList<ToDoList>, name: &str) -> Option<&'a mut ToDoList> {
    for list in lists.iter_mut() {
        if list.title == name {
            return Some(list);
        }
    }
    None
}

// prints all to-do lists with their tasks
fn print_all(lists: &LinkedList<ToDoList>) {
    for list in lists {
        list.print();
        println!("------------------------");
    }
}

// checks that there is at least one to-do list
fn not_empty_lists(lists: &LinkedList<ToDoList>) -> bool {
    if lists.is_empty() {
        println!("\nYou have no to-do lists!");
    }
    return !lists.is_empty()
}

fn main() {
    let mut lists: LinkedList<ToDoList> = LinkedList::new();

    while (true) {
        println!("\nCurrent To-Do Lists:");
        print_all(&lists);

        println!(
            "\nSelect an action:\n\
1. Add a task\n\
2. Remove a task\n\
3. Mark a task as complete\n\
4. Mark a task as incomplete\n\
5. Mark all tasks as complete in a to-do list\n\
6. Mark all tasks as incomplete in a to-do list\n\
7. Add a to-do list\n\
8. Remove a to-do list\n\
9. Exit"
        );

        let choice = input("Enter a number (1-9): ");

        if choice == "1" {
            if not_empty_lists(&lists) {
                let title = input("Enter the list name: ");
                if let Some(list) = find_list_mut(&mut lists, &title) {
                    list.print();
                    let name = input("Enter task name: ");
                    if list.list.iter().any(|task| task.get_name() == name) {
                        println!(
                            "A task with the name '{}' already exists in the list '{}'.",
                            name, list.title
                        );
                    } else {
                        let desc = input("Enter task description: ");
                        list.add_task(Task::new(&name, &desc));
                    }
                } else {
                    println!("List not found.");
                }
            }
        } else if choice == "2" {
            if not_empty_lists(&lists) {
                let title = input("Enter the list name: ");
                if let Some(list) = find_list_mut(&mut lists, &title) {
                    if list.check_empty() {
                        println!("This to-do list is empty. No tasks to remove.");
                    } else {
                        list.print();
                        let name = input("Enter task name to remove: ");
                        list.remove_task(&name);
                    }
                } else {
                    println!("List not found.");
                }
            }
        } else if choice == "3" {
            if not_empty_lists(&lists) {
                let title = input("Enter the list name: ");
                if let Some(list) = find_list_mut(&mut lists, &title) {
                    if list.check_empty() {
                        println!("This to-do list is empty. No tasks to mark complete.");
                    } else {
                        list.print();
                        let name = input("Enter task name to mark complete: ");
                        for task in &mut list.list {
                            if task.get_name() == name {
                                task.mark_complete();
                                break;
                            }
                        }
                    }
                } else {
                    println!("List not found.");
                }
            }
        } else if choice == "4" {
            if not_empty_lists(&lists) {
                let title = input("Enter the list name: ");
                if let Some(list) = find_list_mut(&mut lists, &title) {
                    if list.check_empty() {
                        println!("This to-do list is empty. No tasks to mark incomplete.");
                    } else {
                        list.print();
                        let name = input("Enter task name to mark incomplete: ");
                        for task in &mut list.list {
                            if task.get_name() == name {
                                task.mark_incomplete();
                                break;
                            }
                        }
                    }
                } else {
                    println!("List not found.");
                }
            }
        } else if choice == "5" {
            if not_empty_lists(&lists) {
                let title = input("Enter the list name: ");
                if let Some(list) = find_list_mut(&mut lists, &title) {
                    if list.check_empty() {
                        println!("This to-do list is empty. No tasks to mark complete.");
                    } else {
                        list.mark_all_complete();
                    }
                } else {
                    println!("List not found.");
                }
            }
        } else if choice == "6" {
            if not_empty_lists(&lists) {
                let title = input("Enter the list name: ");
                if let Some(list) = find_list_mut(&mut lists, &title) {
                    if list.check_empty() {
                        println!("This to-do list is empty. No tasks to mark incomplete.");
                    } else {
                        list.mark_all_incomplete();
                    }
                } else {
                    println!("List not found.");
                }
            }
        } else if choice == "7" {
            let name = input("Enter a name for the new to-do list: ");
            if lists.iter().any(|l| l.title == name) {
                println!("\nA list with that name already exists.");
            } else {
                lists.push_back(ToDoList::new(&name));
            }
        } else if choice == "8" {
            if not_empty_lists(&lists) {
                let name = input("Enter the name of the to-do list to remove: ");
                let mut new_lists = LinkedList::new();
                let mut found = false;

                for list in lists.into_iter() {
                    if list.title != name {
                        new_lists.push_back(list);
                    } else {
                        found = true;
                    }
                }

                if !found {
                    println!("List not found.");
                }

                lists = new_lists;
            }
        } else if choice == "9" {
            println!("Goodbye!");
            break;
        } else {
            println!("Invalid input. Try again.");
        }
    }
}
