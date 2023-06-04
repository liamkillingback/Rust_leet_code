use rand::Rng;


#[derive(Debug)]
#[derive(PartialEq)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
    fn get_left_node(self) -> Option<Box<Node>> {
        self.left
    }

    fn get_right_node(self) -> Option<Box<Node>> {
        self.right
    }

    fn add_node(node: Option<Box<Node>>, value: i32) -> Option<Box<Node>> {
        if let Some(mut node) = node {
            if value <= node.value {
                node.left = Node::add_node(node.left, value);
            } else {
                node.right = Node::add_node(node.right, value);
            }
            Some(node)
        } else {
            Some(Box::new(Node::new(value)))
        }
    }
    fn remove_node(root: Option<Box<Node>>, value: i32) -> Option<Box<Node>> {  
        if let Some(mut node) = root {
            if value < node.value {
                node.left = Node::remove_node(node.left, value);
            }
            else if value > node.value {
                node.right = Node::remove_node(node.right, value);
            }
            else { //we found the target node for deletion
                //case of no children
                if node.left.is_none() && node.right.is_none() {
                    return None
                }
                //case of one child
                else if node.left.is_none() {
                    return node.right
                }
                else if node.right.is_none() {
                    return node.left
                }
                //case of two children
                else {
                    let min_node = Node::find_min_node_right_subtree(node.right);
                    node.value = min_node.value;
                    node.right = Node::remove_node(min_node.right, node.value);
                }
            }
            Some(node)
        } else {
            None
        }
    }

    //helper function
    fn find_min_node_right_subtree(mut root:Option<Box<Node>>) -> Box<Node> {
        while let Some(node) = root {
            if node.left.is_none() {
                return node
            }
            else {
                root = node.left;
            }
        }
        root.unwrap()
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut list_head = sample_binary_tree();
    list_head = Node::remove_node(list_head, 12);
    println!("{:#?}", list_head);
    // for n in 1..5 {
    //     let int = rng.gen_range(0..1000);
    //     list_head = Node::add_node(list_head, int); 
    // }


    // let number = 8;
    // let mut result = Box::new(Node::new(0));
    // while let Some(node) = list_head {
    //     if node.value == number {
    //         result = node;
    //         break;
    //     }
    //     if node.value < number {
    //         list_head = node.right;
    //     }
    //     else {
    //         list_head = node.left;
    //     }
    // };
    // println!("{:#?}", result);

}

fn sample_binary_tree() -> Option<Box<Node>> {
    let list = Some(Box::new(Node {
        value: 10,
        left: Some(Box::new(Node {
            value: 8,
            left: None,
            right: None,
        })),
        right: Some(Box::new(Node {
            value: 12,
            left: Some(Box::new(Node {
                value: 11,
                left: None,
                right: None,
            })),
            right: None
        }))
    }));
    list
}