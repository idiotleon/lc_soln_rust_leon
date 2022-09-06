use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/binary-tree-pruning/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        return Self::postorder(root);
    }
    fn postorder(node: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(n) = node {
            let left = n.borrow().left.clone();
            let right = n.borrow().right.clone();
            n.borrow_mut().left = Self::postorder(left.clone());
            n.borrow_mut().right = Self::postorder(right.clone());
            if n.borrow().left.is_none() && n.borrow().right.is_none() && n.borrow().val == 0 {
                return None;
            }
            return Some(n);
        }
        return None;
    }
}
