use std::io;

struct Todo {
    todo_item: String,
}

pub fn run() {
    let mut todos = Vec::new();

    println!("Add an item to the todo list.");
    println!("Tip: use the command 'list' to check the current list.");
    println!("\n");

    loop {
        let mut input = String::new();
        read_line_checker(&mut input);

        match input.trim() {
            "list" => {
                show_todos(&mut todos);
            }
            "List" => {
                show_todos(&mut todos);
            }
            _ => {
                add_todo_item(&mut todos, input);
            }
        }
    }
}

fn read_line_checker(todo_item: &mut String) {
    io::stdin()
        .read_line(todo_item)
        .expect("Failed to read line");
}

fn add_todo_item(todos: &mut Vec<Todo>, todo_item: String) {
    todos.push(Todo { todo_item });
    let last_todo = todos.last().unwrap();

    println!("\n{} has been added.\n", last_todo.todo_item.trim())
}

fn show_todos(todos: &mut Vec<Todo>) {
    println!("\nCurrent list:");

    for todo in todos {
        print!("- {}", todo.todo_item);
    }

    print!("\n");
}
