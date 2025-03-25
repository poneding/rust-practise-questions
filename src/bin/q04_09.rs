/// # Chapter 4 - Enum & Patterns
///
/// Create a Binary tree and have a method `add` on it to add elements to it and `min` and `max` to find the minimum and
/// maximum element in it.
fn main() {
    let mut tree = Node::new(10);
    tree.add(5);
    tree.add(15);
    tree.add(3);
    tree.add(7);
    tree.add(12);
    tree.add(17);

    assert_eq!(tree.min(), 3);
    assert_eq!(tree.max(), 17);
}

struct Node {
    v: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(v: i32) -> Node {
        Node {
            v,
            left: None,
            right: None,
        }
    }

    fn add(&mut self, v: i32) {
        if v < self.v {
            match &mut self.left {
                Some(n) => n.add(v),
                None => self.left = Some(Box::new(Node::new(v))),
            }
        } else {
            match &mut self.right {
                Some(n) => n.add(v),
                None => self.right = Some(Box::new(Node::new(v))),
            }
        }
    }

    fn min(&self) -> i32 {
        match &self.left {
            Some(n) => n.min(),
            None => self.v,
        }
    }

    fn max(&self) -> i32 {
        match &self.right {
            Some(n) => n.max(),
            None => self.v,
        }
    }
}
