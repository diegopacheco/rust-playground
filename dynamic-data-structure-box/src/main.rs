type NodeBox = Option<Box<Node>>;

#[allow(dead_code)]
#[derive(Debug)]
struct Node {
  value: String,
  left : NodeBox,
  right: NodeBox
}

impl Node {
    fn new(s: &str) -> Node {
        Node{value: s.to_string(), left: None, right: None}
    }
    fn boxer(node: Node) -> NodeBox {
        Some(Box::new(node))
    }
    fn set_left(&mut self, node: Node) {
        self.left = Self::boxer(node);
    }
    fn set_right(&mut self, node: Node) {
        self.right = Self::boxer(node);
    }
    fn insert(&mut self, data: &str) {
        if data < &self.value {
            match self.left {
                Some(ref mut n) => n.insert(data),
                None => self.set_left(Self::new(data)),
            }
        } else {
            match self.right {
                Some(ref mut n) => n.insert(data),
                None => self.set_right(Self::new(data)),
            }
        }
    }
}

fn main() {
    let mut root = Node::new("root");
    root.set_left(Node::new("left"));
    root.set_right(Node::new("right"));
    root.insert("one");
    root.insert("two");
    root.insert("five");
    println!("arr {:#?}", root);
}