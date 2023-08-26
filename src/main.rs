mod lib;
use lib::TODO;
use std::io;

fn main() {
    let mut todo = TODO {
        list: Vec::new(),
        num_items: 0,
    };

    println!("Welcome!");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();
        
        let args: Vec<&str> = input.split_whitespace().collect();
        
        if args.is_empty() {
            println!("Invalid command.");
            continue;
        }

        let command = args[0];

        if command == "exit" {
            break; // Exit the loop if the command is "exit"

        } else if command == "add" && args.len() > 1 {
            let item = args[1..].join(" ");
            todo.add(item.to_string());

        } else if command == "done" && args.len() > 1 {
            if let Ok(parsed_index) = args[1].parse::<usize>() {
                todo.done(parsed_index);
            } else {
                println!("Failed to parse the index.");
            }

        } else if command == "list" {
            todo.list();

        } else {
            println!("Invalid command or missing arguments.");
        }
    }
}
