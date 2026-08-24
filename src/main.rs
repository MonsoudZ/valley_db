use std::collections::HashMap;
use std::io;

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
            let key = input_key();
            let value = input_value();
            match insert(&mut mapping, key, value) {
                Some(value) => {
                    println!(" Old Value was replaced {value} with new Value");
                }
                None => println!("New record for was created"),
            }
        } else if user_action == "delete" {
            let key = input_key();
            match delete_entry_by_key(&mut mapping, key) {
                Some(value) => {
                    println!("Value: {value} was deleted");
                }
                None => println!("No key found, try again."),
            }
        } else if user_action == "get" {
            let get_key = input_key();
            match get_value(&mapping, get_key) {
                Some(value) => {
                    println!("Value: {value} was found");
                }
                None => println!("No key found, try again."),
            }
        } else if user_action == "exit" {
            break;
        } else {
            println!("Unknown action");
        }
    }
}

fn insert(storage: &mut HashMap<String, String>, key: String, value: String) -> Option<String> {
    storage.insert(key, value)
}

fn delete_entry_by_key(storage: &mut HashMap<String, String>, key: String) -> Option<String> {
    storage.remove(&key)
}

fn get_value(storage: &HashMap<String, String>, key: String) -> Option<&String> {
    storage.get(&key)
}

fn input_key() -> String {
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
fn input_value() -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut storage = HashMap::new();

        insert(
            &mut storage,
            String::from("cheese_type"),
            String::from("blue cheese"),
        );

        assert_eq!(
            storage.get("cheese_type"),
            Some(&String::from("blue cheese"))
        );
    }

    #[test]
    fn test_delete_entry_by_key() {
        let mut storage = HashMap::new();
        insert(
            &mut storage,
            String::from("cheese_type"),
            String::from("blue cheese"),
        );
        delete_entry_by_key(&mut storage, String::from("cheese_type"));

        assert_eq!(storage.get("cheese_type"), None);
    }

    #[test]
    fn test_get_missing_key() {
        let mut storage = HashMap::new();
        insert(
            &mut storage,
            String::from("cheese_type"),
            String::from("blue cheese"),
        );
        let result = get_value(&storage, String::from("toys"));

        assert_eq!(result, None);
    }

    #[test]
    fn test_get_key() {
        let mut storage = HashMap::new();
        insert(
            &mut storage,
            String::from("cheese_type"),
            String::from("blue cheese"),
        );
        let result = get_value(&storage, String::from("cheese_type"));

        assert_eq!(result, Some(&String::from("blue cheese")));
    }

    #[test]

    fn test_insert_overwrites_existing_key() {
        let mut storage = HashMap::new();
        let old = insert(
            &mut storage,
            String::from("cheese_type"),
            String::from("blue cheese"),
        );
        let new = insert(
            &mut storage,
            String::from("cheese_type"),
            String::from("cheddar cheese"),
        );

        assert_eq!(old, None);
        assert_eq!(new, Some(String::from("blue cheese")));
        assert_eq!(
            storage.get("cheese_type"),
            Some(&String::from("cheddar cheese"))
        );
    }

    #[test]
    fn test_insert_multiple_key() {
        let mut storage = HashMap::new();
        insert(
            &mut storage,
            String::from("cheese_type"),
            String::from("blue cheese"),
        );
        insert(&mut storage, String::from("sports"), String::from("soccer"));

        assert_eq!(
            storage.get("cheese_type"),
            Some(&String::from("blue cheese"))
        );
        assert_eq!(storage.get("sports"), Some(&String::from("soccer")));
    }

    #[test]
    fn test_delete_missing_key() {
        let mut storage = HashMap::new();
        insert(
            &mut storage,
            String::from("cheese_type"),
            String::from("blue cheese"),
        );
        let result = delete_entry_by_key(&mut storage, String::from("sports"));

        assert_eq!(result, None);
    }
}
