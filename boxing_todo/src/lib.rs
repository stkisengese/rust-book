mod err;

pub use err::{ParseErr, ReadErr};
pub use std::{error::Error, fs::File, io::Read};
use std::fs;
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
        // File handling with proper ReadErr
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
            .ok_or(Box::new(ParseErr::Empty))?
            .to_string();

        // Tasks processing
        let tasks = parsed["tasks"].members()
            .map(|task| {
                Ok(Task {
                    id: task["id"].as_u32().ok_or(Box::new(ParseErr::Empty))?,
                    description: task["description"].as_str()
                        .ok_or(Box::new(ParseErr::Empty))?
                        .to_string(),
                    level: task["level"].as_u32().ok_or(Box::new(ParseErr::Empty))?,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        // Empty tasks check
        if tasks.is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }

        Ok(TodoList { title, tasks })
    }
}