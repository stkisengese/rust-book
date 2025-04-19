#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>, 
}

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,  
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        self.head = Some(Node {
            value,
            next: self.head.take().map(Box::new),
        });
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        self.head.take().map(|node| {
            self.head = node.next.map(|boxed| *boxed);
            node.value
        })
    }

    // pub fn len(&self) -> usize {
    //     let mut count = 0;
    //     let mut current = &self.head;
    //     while let Some(node) = current {
    //         count += 1;
    //         // Create a temporary Option<Node<T>> for the next iteration
    //         current = match &node.next {
    //             Some(boxed_node) => {
    //                 // Create a new Option<Node<T>> by dereferencing the Box
    //                 unsafe {
    //                     // SAFETY: We're just creating a temporary reference for counting
    //                     // and won't use it after this scope
    //                     let next_node = &**boxed_node;
    //                     &*(next_node as *const Node<T> as *const Option<Node<T>>)
    //                 }
    //             },
    //             None => &None,
    //         };
    //     }
    //     count
    // }
    
    pub fn len(&self) -> usize {
    let mut count = 0;
    
    // Define a recursive helper function
    fn count_next<T>(next_opt: &Option<Box<Node<T>>>) -> usize {
        match next_opt {
            None => 0,
            Some(boxed_node) => 1 + count_next(&boxed_node.next),
        }
    }
    
    // Count the head if it exists
    if let Some(head) = &self.head {
        count = 1 + count_next(&head.next);
    }
    
    count
}
}