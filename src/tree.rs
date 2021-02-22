pub(crate) mod includes;
use includes::*;
use crate::tree::traversal::Traversal;

type Node<T> = Rc<RefCell<TreeNode<T>>>;

pub mod treenode;
pub mod traversal;
pub mod operations;

#[derive(Debug, PartialEq, Eq)]
pub struct Tree<T> where T: Copy + Clone + Debug {
    root: Option<Node<T>>,
}

impl<T> Tree<T> where T: Copy + Clone + Debug {
    pub(in crate::tree) fn new() -> Self {
        Self { root: None }
    }
}
impl<T> ToString for Tree<T> where T: Copy + Clone + Debug {
    fn to_string(&self) -> String {
        fn recursive<T: Copy + Debug>(node: &Option<Node<T>>) -> String {
            match node {
                None => String::new(),
                Some(node) => {
                    let node = node.borrow();
                    let left = match node.left.is_some() {
                        true => format!("({})", recursive(&node.left)),
                        false => String::new(),
                    };
                    let right = match node.right.is_some() {
                        true => format!("({})", recursive(&node.right)),
                        false => String::new(),
                    };

                    format!("{:?}{}{}", node.value, left, right)
                }
            }
        }

        recursive(&self.root)
    }
}
pub mod nodebuilder;
pub mod treebuilder;
