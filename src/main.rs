use clap::Parser;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::{fs, path::Path};

const FILE_PATH: &str = "todo.json";

/// A simple command-line TODO list manager
#[derive(Parser)]
struct Cli {
    /// Command to run (e.g., add, list, mark-done)
    command: String,

    /// Optional key for the command (e.g., the task name)
    key: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct TodoList {
    items: HashMap<String, bool>,
}

impl TodoList {
    #[cfg(not(test))]
    fn new() -> Self {
        if Path::new(FILE_PATH).exists() {
            let data = fs::read_to_string(FILE_PATH).unwrap_or_default();
            serde_json::from_str(&data).unwrap_or_else(|_| TodoList {
                items: HashMap::new(),
            })
        } else {
            TodoList {
                items: HashMap::new(),
            }
        }
    }

    #[cfg(test)]
    fn new() -> Self {
        TodoList {
            items: HashMap::new(),
        }
    }

    #[cfg(not(test))]
    fn save(&self) {
        let json = serde_json::to_string_pretty(&self).unwrap();
        fs::write(FILE_PATH, json).unwrap();
    }

    #[cfg(test)]
    fn save(&self) {
        // No-op during tests
    }

    fn add(&mut self, key: String) {
        self.items.entry(key).or_insert(true);
        self.save();
    }

    fn mark(&mut self, key: String, value: bool) -> Result<(), String> {
        let x = self.items.get_mut(&key).ok_or(key.clone())?;
        *x = value;
        self.save();
        Ok(())
    }

    fn list(&self) -> (impl Iterator<Item = &String>, impl Iterator<Item = &String>) {
        (
            self.items.iter().filter(|x| *x.1 == true).map(|x| x.0),
            self.items.iter().filter(|x| *x.1 == false).map(|x| x.0),
        )
    }
}

fn main() {
    let args = Cli::parse();
    let mut todo = TodoList::new();

    let result = match args.command.as_str() {
        "add" => match args.key {
            Some(key) => {
                todo.add(key);
                Ok(())
            }
            None => Err("Key cannot be empty!".to_string()),
        },
        "mark-done" => match args.key {
            Some(key) => todo
                .mark(key, false)  // ⬅️ Correct: mark it as done
                .map_err(|e| format!("Invalid key: {}", e))
                .map(|_| ()),
            None => Err("Key cannot be empty!".to_string()),
        },
        "list" => {
            let (todo_items, done_items) = todo.list();

            println!("# TO DO\n");
            todo_items.for_each(|x| println!(" * {}", x));

            println!("\n# DONE\n");
            done_items.for_each(|x| println!(" * {}", x));

            Ok(())
        },
        "clear" => {
            todo.items.clear();
            todo.save();
            Ok(())
        }
        cmd => Err(format!("Command '{}' not recognized", cmd)),
    };

    match result {
        Err(e) => println!("ERROR: {}", e),
        Ok(_) => println!("SUCCESS"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_todo() {
        let todo = TodoList::new();
        assert!(todo.items.is_empty());
    }

    #[test]
    fn add_item() {
        let mut todo = TodoList::new();
        todo.add("Something to do".to_string());
        assert_eq!(todo.items.get("Something to do"), Some(&true));
    }

    #[test]
    fn add_items_already_exist() {
        let mut todo = TodoList::new();
        todo.add("Something to do".to_string());
        todo.add("Something to do".to_string());
        assert_eq!(todo.items.get("Something to do"), Some(&true));
        assert_eq!(todo.items.len(), 1);
    }

    #[test]
    fn add_item_does_not_change_value() {
        let mut todo = TodoList::new();
        todo.add("Something to do".to_string());

        if let Some(x) = todo.items.get_mut("Something to do") {
            *x = false;
        }

        todo.add("Something to do".to_string());
        assert_eq!(todo.items.get("Something to do"), Some(&false));
        assert_eq!(todo.items.len(), 1);
    }

    #[test]
    fn mark_item() {
        let mut todo = TodoList::new();
        todo.add("Something to do".to_string());
        assert!(todo.mark("Something to do".to_string(), false).is_ok());
        assert_eq!(todo.items.get("Something to do"), Some(&false));
        assert!(todo.mark("Something to do".to_string(), true).is_ok());
        assert_eq!(todo.items.get("Something to do"), Some(&true));
    }

    #[test]
    fn mark_item_does_not_exist() {
        let mut todo = TodoList::new();
        assert_eq!(
            todo.mark("Something to do".to_string(), false),
            Err("Something to do".to_string())
        );
    }

    #[test]
    fn list_items() {
        let mut todo = TodoList::new();
        todo.add("Something to do".to_string());
        todo.add("Something else to do".to_string());
        todo.add("Something done".to_string());
        todo.mark("Something done".to_string(), false).unwrap();

        let (todo_items, done_items) = todo.list();

        let todo_items: Vec<String> = todo_items.cloned().collect();
        let done_items: Vec<String> = done_items.cloned().collect();

        assert!(todo_items.contains(&"Something to do".to_string()));
        assert!(todo_items.contains(&"Something else to do".to_string()));
        assert_eq!(todo_items.len(), 2);
        assert!(done_items.contains(&"Something done".to_string()));
        assert_eq!(done_items.len(), 1);
    }
}

