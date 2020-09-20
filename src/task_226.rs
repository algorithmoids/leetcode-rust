// Definition for a binary tree node.

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;

struct Solution {}

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        root.map(|tree| {
            let node = tree.borrow();
            let mut inverted = TreeNode::new(node.val);
            inverted.left = Solution::invert_tree(node.right.clone());
            inverted.right = Solution::invert_tree(node.left.clone());

            Rc::new(RefCell::new(inverted))
        })
    }
}


#[test]
fn test() {
    let tree = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    println!("{:?}", Solution::invert_tree(tree))
}
