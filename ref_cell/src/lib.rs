pub mod messenger;

pub use std::cell::RefCell;
pub use std::collections::HashMap;
pub use std::rc::Rc;
pub use messenger::Logger;
pub use messenger::Tracker;

pub struct Worker {
    pub track_value: Rc<usize>,
    pub all_messages: RefCell<Vec<String>>,
    pub mapped_messages: RefCell<HashMap<String, String>>,
}

impl Worker {
    // new: that initializes a Worker structure.
    pub fn new(value: usize) -> Self {
        Worker { 
            track_value: Rc::new(value), 
            all_messages: RefCell::new(Vec::new()), 
            mapped_messages: RefCell::new(HashMap::new())
         }
    }    
}

impl Logger for Worker {
    fn warning(&self, msg: &str) {
        let full_msg = format!("Warning: {}", msg);
        self.all_messages.borrow_mut().push(full_msg.to_string());
        self.mapped_messages.borrow_mut().insert("Warning".to_string(), msg.to_string());
    }

    fn info(&self, msg: &str) {
        let full_msg = format!("Info: {}", msg);
        self.all_messages.borrow_mut().push(full_msg.to_string());
        self.mapped_messages.borrow_mut().insert("Info".to_string(), msg.to_string());
    }

    fn error(&self, msg: &str) {
        let full_msg = format!("Error: {}", msg);
        self.all_messages.borrow_mut().push(full_msg.to_string());
        self.mapped_messages.borrow_mut().insert("Error".to_string(), msg.to_string());
    }
}
