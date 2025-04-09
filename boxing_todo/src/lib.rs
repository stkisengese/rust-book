mod err;

pub use err::{ParseErr, ReadErr};
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
        
         // Validate JSON structure
         if !parsed.has_key("title") || !parsed.has_key("tasks") {
            return Err(Box::new(err::ParseErr::Malformed(Box::new(std::fmt::Error))));
        }

        // Extract title
        let title = parsed["title"].as_str()
            .ok_or_else(|| Box::new(err::ParseErr::Malformed(Box::new(std::fmt::Error))))?
            .to_string();

        // Extract tasks
        let tasks_json = &parsed["tasks"];
        
        // Check if tasks array is empty
        if tasks_json.is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }

        // Parse tasks
        let mut tasks = Vec::new();
        for task in parsed["tasks"].members() {
            let id = task["id"].as_u32()
                .ok_or_else(|| Box::new(err::ParseErr::Malformed(Box::new(std::fmt::Error))))?;

            let description = task["description"].as_str()
                .ok_or_else(|| Box::new(err::ParseErr::Malformed(Box::new(std::fmt::Error))))?
                .to_string();

            let level = task["level"].as_u32()
                .ok_or_else(|| Box::new(err::ParseErr::Malformed(Box::new(std::fmt::Error))))?;

            tasks.push(Task {
                id,
                description,
                level,
            });
        }

        Ok(TodoList {
            title,
            tasks,
        })
    }
}
