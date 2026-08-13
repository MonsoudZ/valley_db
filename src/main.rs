use std::io;
use std::collections::HashMap;

fn main() {
    let mut mapping = HashMap::new();

    loop {
        println!("Input your desired function");

        let mut user_action = String::new();
        io::stdin()
            .read_line(&mut user_action)
            .expect("Not a valid function call");

        let user_action = user_action.trim().to_lowercase();

        if user_action == "insert" {
            insert(&mut mapping)
        }
        else if user_action == "delete" {
            delete_from_value(&mut mapping)
        }
        else if user_action == "get" {
            get_value_for(& mapping)
        }
        else if user_action == "exit" {
            break;
        }
        else {
            println!("Unknown action");
        }
    }
}

fn insert(i: &mut HashMap<String, String>) {
    i.insert(get_key(),get_value());
}

fn delete_from_value(i: &mut HashMap<String, String>) {
    match i.remove(&(get_key())) {
        Some(value) => {
            println!("Value: {value} was deleted");
        }
        None => println!("No key found, try again."),
    }
}

fn get_value_for(i: &HashMap<String, String>) {
    println!("Input key name");

    let mut user_key = String::new();
    io::stdin()
        .read_line(&mut user_key)
        .expect("Failed to read line");
    let user_key = user_key.trim();

    match i.get(user_key) {
        Some(value) => {
            println!("Value: {value}");
        }
        None => println!("No key '{user_key}' found, try again."),
    }
}

fn get_key() -> String {
    loop {
        println!("Input key name");
        let mut key = String::new();

        io::stdin()
            .read_line(&mut key)
            .expect("Failed to read line");

        let cleaned_key = key.trim();

        if cleaned_key.is_empty() {
            println!("empty key, try again.");
        } else {
            return cleaned_key.to_string();
        }
    }
}
fn get_value() -> String {
    loop {
        println!("Input Value");
        let mut value = String::new();

        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read line");

         let cleaned_value = value.trim();

        if cleaned_value.is_empty() {
            println!("empty value, try again.");
        } else {
            return cleaned_value.to_string();
        }
    }
}


