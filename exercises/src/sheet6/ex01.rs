#![procedural::magic_macro]
//! Create a struct `TreeNode` generic over `T` that represents a binary tree.
//! It should have a field `value` of type `T` and two optional fields `left` and `right` (they
//! should hold a pointer to another `TreeNode`).
//! Implement:
//! - a method `new` that takes a value and returns a new `TreeNode` with the given value and no children.
//! - a method `from_vec` that takes a vector of values and returns a `TreeNode` with the given values.
//! - a method `insert` that takes a value and inserts it into the tree (follow binary search tree rules).
//!
//! Implement the preorder, inorder and postorder traversal algorithms for the tree.
//!
//! Keep in mind that the type `T` must implement the `PartialOrd` and `Clone` trait.

use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T: PartialOrd + Clone + Debug> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}
#[runtest(1.0)]
///checks if the new function works correctly for some value (and that the type is generic enough)
fn test_new(){
    use std::cmp::Ordering::Equal;
    let node = TreeNode::new(0);
    assert!(node.left.is_none() && node.right.is_none() && node.value==0);
    let node = TreeNode::new(String::new());
    assert!(node.left.is_none() && node.right.is_none() && node.value==String::new());
    fn generic_new<T: PartialOrd + Clone + Debug>(t: T){
        let new_tree = TreeNode::new(t);
    }
}

#[runtest(1.0)]
///checks if from_vec associated function works as expected
fn test_from_vec(){
    let t = vec![0, 4, 2, 3, 4, 1, 3];
    
}

impl<T> TreeNode<T>
where
    T: PartialOrd + Clone + Debug,
{
    pub fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    pub fn from_vec(vec: &[T]) -> Self {
        let mut tree = TreeNode::new(vec[0].clone());
        for value in vec.iter().skip(1) {
            tree.insert(value.clone());
        }
        tree
    }

    pub fn insert(&mut self, value: T) {
        if value < self.value {
            match self.left {
                Some(ref mut left) => left.insert(value),
                None => self.left = Some(Box::new(TreeNode::new(value))),
            }
        } else {
            match self.right {
                Some(ref mut right) => right.insert(value),
                None => self.right = Some(Box::new(TreeNode::new(value))),
            }
        }
    }

    pub fn preorder(&self) {
        println!("{:?}", self.value);
        if let Some(ref left) = self.left {
            left.preorder();
        }
        if let Some(ref right) = self.right {
            right.preorder();
        }
    }

    pub fn inorder(&self) {
        if let Some(ref left) = self.left {
            left.inorder();
        }
        println!("{:?}", self.value);
        if let Some(ref right) = self.right {
            right.inorder();
        }
    }

    pub fn postorder(&self) {
        if let Some(ref left) = self.left {
            left.postorder();
        }
        if let Some(ref right) = self.right {
            right.postorder();
        }
        println!("{:?}", self.value);
    }
}


#[test]
fn normal_tree() {
    let mut tree = TreeNode::new(4);
    tree.insert(2);
    tree.insert(5);

    println!("{:?}", tree);
}

#[test]
fn tree_from_vec() {
    let vec = vec!['d', 'c', 'b', 'a', 'e', 'g', 'f'];
    let tree = TreeNode::from_vec(&vec);

    println!("{:?}", tree);
}

#[test]
fn tree_preorder() {
    let vec = vec!['d', 'c', 'b', 'a', 'e', 'g', 'f'];
    let tree = TreeNode::from_vec(&vec);

    tree.preorder();
}

#[test]
fn tree_inorder() {
    let vec = vec![10, 3, 5, 2, 1, 4, 6, 7];
    let tree = TreeNode::from_vec(&vec);

    tree.inorder();
}

#[test]
fn tree_postorder() {
    let vec = vec!['d', 'c', 'b', 'a', 'e', 'g', 'f'];
    let tree = TreeNode::from_vec(&vec);

    tree.postorder();
}

