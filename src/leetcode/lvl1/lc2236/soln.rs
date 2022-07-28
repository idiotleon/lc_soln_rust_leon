use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/root-equals-sum-of-children/
/// Time Complexity:    O(1)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(rt) = root {
            let value = rt.borrow().val;
            let left = rt.borrow().left.clone().unwrap().borrow().val;
            let right = rt.borrow().right.clone().unwrap().borrow().val;
            return value == left + right;
        }
        true
    }
}
