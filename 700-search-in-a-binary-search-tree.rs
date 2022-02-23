use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering::{Less, Equal, Greater};

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
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

struct Solution {

}

impl Solution {
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        use std::cmp::Ordering;
        if let Some(t) = root {
            let cur_val = t.borrow().val;
            match cur_val.cmp(&val) {
                Ordering::Equal => Some(t),
                Ordering::Greater => Self::search_bst(t.borrow_mut().left.take(), val),
                Ordering::Less => Self::search_bst(t.borrow_mut().right.take(), val),
            }
        } else {None}
    }
}

impl Solution {
    pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) {
        println!("{:?}", root.as_ref().unwrap().borrow().val);
    }
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let res = Solution::search_bst(root, 1);
    Solution::print_tree(res);
}
