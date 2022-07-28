use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/evaluate-boolean-binary-tree/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::postorder(root)
    }
    fn postorder(node: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(n) = node {
            let value = n.borrow().val;
            if n.borrow().left.is_none() && n.borrow().right.is_none() {
                return value == 1;
            }
            let left = Self::postorder(n.borrow().left.clone());
            let right = Self::postorder(n.borrow().right.clone());
            return if value == 2 {
                left || right
            } else {
                left && right
            };
        }
        unreachable!();
    }
}
