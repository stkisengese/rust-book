mod err;
pub use err::{ParseErr, ReadErr};

use std::error::Error;
use std::fs::File;
use std::io::Read;
extern crate json;

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
        // File handling
        let mut file = File::open(path)
            .map_err(|e| Box::new(ReadErr { child_err: Box::new(e) }))?;
        
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| Box::new(ReadErr { child_err: Box::new(e) }))?;

        // Early empty check
        if content.trim().is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }

        // JSON parsing
        let parsed = json::parse(&content)
            .map_err(|e| Box::new(ParseErr::Malformed(Box::new(e))))?;

        // Title extraction
        let title = parsed["title"].as_str()
            .ok_or_else(|| Box::new(ParseErr::Empty))?
            .to_string();

        // Tasks processing
        let mut tasks = Vec::new();
        for task in parsed["tasks"].members() {
            tasks.push(Task {
                id: task["id"].as_u32()
                    .ok_or_else(|| Box::new(ParseErr::Empty))?,
                description: task["description"].as_str()
                    .ok_or_else(|| Box::new(ParseErr::Empty))?
                    .to_string(),
                level: task["level"].as_u32()
                    .ok_or_else(|| Box::new(ParseErr::Empty))?,
            });
        }

        // Empty tasks check
        if tasks.is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }

        Ok(TodoList { title, tasks })
    }
}