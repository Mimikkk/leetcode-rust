use crate::tree::*;

pub enum TraversalDirection { Preorder, Inorder, Postorder, Levelorder, Spiralorder }
pub trait Traversal<T> where T: Copy + Clone + Debug {
    fn traverse(&self, direction: TraversalDirection)
                -> Either<Vec<Node<T>>, Vec<Vec<Node<T>>>> {
        match direction {
            TraversalDirection::Preorder => Left(self.preorder()),
            TraversalDirection::Inorder => Left(self.inorder()),
            TraversalDirection::Postorder => Left(self.postorder()),
            TraversalDirection::Levelorder => Right(self.levelorder()),
            TraversalDirection::Spiralorder => Left(self.spiralorder()),
        }
    }

    fn traverse_values(&self, direction: TraversalDirection) -> Either<Vec<T>, Vec<Vec<T>>> {
        match self.traverse(direction) {
            Left(nodes) => Left(nodes.into_iter().map(|n|
                n.as_ref().borrow().value).collect()),
            Right(levels) => Right(levels.into_iter().map(|l|
                l.into_iter().map(|n| n.as_ref().borrow().value).collect()).collect()),
        }
    }

    fn preorder(&self) -> Vec<Node<T>>;
    fn inorder(&self) -> Vec<Node<T>>;
    fn postorder(&self) -> Vec<Node<T>>;
    fn levelorder(&self) -> Vec<Vec<Node<T>>>;
    fn spiralorder(&self) -> Vec<Node<T>>;
}
impl<T> Traversal<T> for Tree<T> where T: Clone + Copy + Debug + PartialEq {
    fn preorder(&self) -> Vec<Node<T>> {
        let mut stack = Vec::new();
        let mut result = Vec::new();

        stack.push(self.root.clone());

        while let Some(node) = stack.pop() {
            if let Some(node) = node {
                result.push(node.clone());
                stack.push(node.as_ref().borrow().right.clone());
                stack.push(node.as_ref().borrow().left.clone());
            }
        }
        result
    }

    fn inorder(&self) -> Vec<Node<T>> {
        let mut result = Vec::new();
        let mut stack = Vec::new();

        let mut node = self.root.clone();
        while !stack.is_empty() || node.is_some() {
            match node {
                Some(n) => {
                    stack.push(Some(n.clone()));
                    node = n.as_ref().borrow().left.clone();
                }
                None => if let Some(Some(n)) = stack.pop() {
                    result.push(n.clone());
                    node = n.as_ref().borrow().right.clone();
                },
            }
        }

        result
    }

    fn postorder(&self) -> Vec<Node<T>> {
        let mut stack = Vec::new();
        let mut result = Vec::new();
        let mut node = self.root.clone();

        loop {
            while node.is_some() {
                stack.push(node.clone());
                stack.push(node.clone());
                node = node.unwrap().as_ref().borrow().left.clone();
            }

            match stack.pop() {
                Some(n) => node = n,
                None => return result,
            }

            let is_right_processed = || {
                stack.last().is_some() && *stack.last().unwrap() == node
            };
            node = match is_right_processed() {
                true => node.unwrap().as_ref().borrow().right.clone(),
                false => {
                    result.push(node.unwrap().clone());
                    None
                }
            }
        }
    }

    fn levelorder(&self) -> Vec<Vec<Node<T>>> {
        match self.root {
            None => Vec::new(),
            Some(_) => {
                let mut level = VecDeque::new();
                let mut levels = Vec::new();

                level.push_front(self.root.clone());
                while level.iter().any(|x| x.is_some()) {
                    levels.push(Vec::new());
                    let mut next_level = Vec::new();

                    while let Some(node) = level.pop_front() {
                        if let Some(node) = node {
                            levels.last_mut().unwrap().push(node.clone());

                            let node = node.as_ref().borrow();
                            next_level.push(node.left.clone());
                            next_level.push(node.right.clone());
                        }
                    }

                    level.extend(next_level);
                }
                levels
            }
        }
    }

    fn spiralorder(&self) -> Vec<Node<T>> {
        self.levelorder().into_iter().fold(
            (Vec::new(), false),
            |(acc, is_even), row| match is_even {
                true => ([acc, row.into_iter().rev().collect()].concat(), false),
                false => ([acc, row].concat(), true),
            }).0
    }
}
