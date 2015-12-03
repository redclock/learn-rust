use std::boxed::Box;
enum LinkedListNext {
    Next(Box<LinkedNode>),
    Nil
}

impl LinkedListNext {
    fn new_next(val: i32) -> LinkedListNext {
        LinkedListNext::Next(
                    Box::new(LinkedNode::new(val))
                )    
    }
}

struct LinkedNode {
    value: i32,
    next: LinkedListNext
}

impl LinkedNode {
    fn new(value: i32) -> LinkedNode {
        return LinkedNode { value: value, next: LinkedListNext::Nil }      
    }

    fn append(&mut self, val: i32) {
        match self.next {
            LinkedListNext::Next(ref mut node) => {
                node.append(val);
            } 
            LinkedListNext::Nil => {
                self.next = LinkedListNext::new_next(val);
            }
        }
    }

    fn insert(&mut self, val: i32, depth: i32) {
        match self.next {
            LinkedListNext::Next(ref mut node) => {
                if depth <= 0 {
                }
                else {
                    node.insert(val, depth - 1);
                }
            } 
            LinkedListNext::Nil => {
                return
            }
        }

        self.next = LinkedListNext::Next(
            Box::new(LinkedNode  { value: val, next: self.next })
        );   

    }

}

struct LinkedList {
    head: Box<LinkedNode>,
}

impl LinkedList {
    fn new() -> LinkedList {
        return LinkedList {
           head: Box::new(LinkedNode::new(-1))     
        }
    }
    
    fn output(&self) {
        let mut cur_node = &self.head;
        print!("Head");
        loop {
            match cur_node.next {
                LinkedListNext::Next(ref node) => {
                    cur_node = &node;
                    print!("->{}", cur_node.value);
                } 
                LinkedListNext::Nil => {
                    println!("");
                    return
                }
            }
        }    
    }
    
    fn append(&mut self, val: i32) {
        self.head.append(val);
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.append(2);
    list.append(3);
    list.append(4);
    list.append(5);
    list.output();
    println!("Hello, world!");
}
