use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
/// @author: Leon
/// https://leetcode.com/problems/maximum-depth-of-binary-tree/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::postorder(root) as i32
    }
    fn postorder(node: Option<Rc<RefCell<TreeNode>>>) -> u16 {
        if let Some(n) = node {
            let left = Self::postorder(n.borrow().left.clone());
            let right = Self::postorder(n.borrow().right.clone());
            return 1 + std::cmp::max(left, right);
        }
        0
    }
}
