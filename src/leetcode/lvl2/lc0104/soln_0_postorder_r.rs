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
        Self::postorder(root)
    }
    fn postorder(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if node.is_none() {
            return 0;
        }
        let node = node.unwrap();
        let left = Self::postorder(node.borrow().left.clone());
        let right = Self::postorder(node.borrow().right.clone());
        std::cmp::max(left, right) + 1
    }
}
