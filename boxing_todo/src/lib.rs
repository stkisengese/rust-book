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
        // File handling - no unwrap
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(e) => return Err(Box::new(ReadErr { child_err: Box::new(e) })),
        };
        
        let mut content = String::new();
        match file.read_to_string(&mut content) {
            Ok(_) => {},
            Err(e) => return Err(Box::new(ReadErr { child_err: Box::new(e) })),
        }

        // Early empty check
        if content.trim().is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }

        // JSON parsing - no unwrap
        let parsed = match json::parse(&content) {
            Ok(p) => p,
            Err(e) => return Err(Box::new(ParseErr::Malformed(Box::new(e)))),
        };

        // Title extraction - no unwrap
        let title = match parsed["title"].as_str() {
            Some(t) => t.to_string(),
            None => return Err(Box::new(ParseErr::Empty)),
        };

        // Tasks processing - no unwrap
        let mut tasks = Vec::new();
        for task in parsed["tasks"].members() {
            let id = match task["id"].as_u32() {
                Some(id) => id,
                None => return Err(Box::new(ParseErr::Empty)),
            };
            
            let description = match task["description"].as_str() {
                Some(d) => d.to_string(),
                None => return Err(Box::new(ParseErr::Empty)),
            };
            
            let level = match task["level"].as_u32() {
                Some(l) => l,
                None => return Err(Box::new(ParseErr::Empty)),
            };
            
            tasks.push(Task { id, description, level });
        }

        // Empty tasks check
        if tasks.is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }

        Ok(TodoList { title, tasks })
    }
}