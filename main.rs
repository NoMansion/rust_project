mod task;
mod todo_list;

use std::collections::LinkedList;
use std::io::{self, Write};
use task::Task;
use todo_list::ToDoList;

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    let _ = io::stdout().flush();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

fn find_list_mut<'a>(lists: &'a mut LinkedList<ToDoList>, name: &str) -> Option<&'a mut ToDoList> {
    for list in lists.iter_mut() {
        if list.day == name {
            return Some(list);
        }
    }
    None
}

fn print_all(lists: &LinkedList<ToDoList>) {
    for list in lists {
        list.print();
        println!("------------------------");
    }
}

fn main() {
    let mut lists: LinkedList<ToDoList> = LinkedList::new();
    for day in [
        "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday",
    ] {
        lists.push_back(ToDoList::new(day));
    }

    loop {
        println!("\nCurrent To-Do Lists:");
        print_all(&lists);

        println!(
            "\nSelect an action:
1. Add a task
2. Remove a task
3. Mark a task as complete
4. Mark a task as incomplete
5. Mark all tasks as complete
6. Mark all tasks as incomplete
7. Add a to-do list
8. Remove a to-do list
9. Exit"
        );

        let choice = input("Enter a number (1-9): ");

        match choice.as_str() {
            "1" => {
                let day = input("Enter the list name: ");
                if let Some(list) = find_list_mut(&mut lists, &day) {
                    list.print(); // Show tasks before action
                    let name = input("Enter task name: ");
                    let desc = input("Enter task description: ");
                    list.add_task(Task::new(&name, &desc));
                } else {
                    println!("List not found.");
                }
            }
            "2" => {
                let day = input("Enter the list name: ");
                if let Some(list) = find_list_mut(&mut lists, &day) {
                    list.print(); // Show tasks before action
                    let name = input("Enter task name to remove: ");
                    list.remove_task(&name);
                } else {
                    println!("List not found.");
                }
            }
            "3" => {
                let day = input("Enter the list name: ");
                if let Some(list) = find_list_mut(&mut lists, &day) {
                    list.print(); // Show tasks before action
                    let name = input("Enter task name to mark complete: ");
                    for task in &mut list.list {
                        if task.get_name() == name {
                            task.mark_complete();
                            break;
                        }
                    }
                } else {
                    println!("List not found.");
                }
            }
            "4" => {
                let day = input("Enter the list name: ");
                if let Some(list) = find_list_mut(&mut lists, &day) {
                    list.print(); // Show tasks before action
                    let name = input("Enter task name to mark incomplete: ");
                    for task in &mut list.list {
                        if task.get_name() == name {
                            task.mark_uncomplete();
                            break;
                        }
                    }
                } else {
                    println!("List not found.");
                }
            }
            "5" => {
                let day = input("Enter the list name: ");
                if let Some(list) = find_list_mut(&mut lists, &day) {
                    list.mark_all_complete();
                } else {
                    println!("List not found.");
                }
            }
            "6" => {
                let day = input("Enter the list name: ");
                if let Some(list) = find_list_mut(&mut lists, &day) {
                    list.mark_all_uncomplete();
                } else {
                    println!("List not found.");
                }
            }
            "7" => {
                let name = input("Enter a name for the new to-do list: ");
                if lists.iter().any(|l| l.day == name) {
                    println!("A list with that name already exists.");
                } else {
                    lists.push_back(ToDoList::new(&name));
                }
            }
            "8" => {
                let name = input("Enter the name of the to-do list to remove: ");
                let mut new_lists = LinkedList::new();
                let mut found = false;

                for list in lists.into_iter() {
                    if list.day != name {
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
            "9" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid input. Try again."),
        }
    }
}
