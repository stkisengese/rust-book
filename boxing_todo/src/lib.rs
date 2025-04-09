mod err;

use err::{ParseErr};
pub use json::{object, JsonValue};
use std::{error::Error, fs};

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let content = fs::read_to_string(path)
            .map_err(|e| Box::new(err::ReadErr { child_err: Box::new(e) }))?;

        let parsed = json::parse(&content)
            .map_err(|e| Box::new(err::ParseErr::Malformed(Box::new(e))))?;
        
        // Extract title
        let title = parsed["title"].as_str().unwrap_or("").to_string();

        // Extract tasks
        let tasks_json = &parsed["tasks"];
        
        // Check if tasks array is empty
        if tasks_json.is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }

        // Parse tasks
        let mut tasks = Vec::new();
        for task in tasks_json.members() {
            tasks.push(Task {
                id: task["id"].as_u32().unwrap_or(0),
                description: task["description"].as_str().unwrap_or("").to_string(),
                level: task["level"].as_u32().unwrap_or(0),
            });
        }

        Ok(TodoList {
            title,
            tasks,
        })
    }
}
