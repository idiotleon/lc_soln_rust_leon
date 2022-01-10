use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/count-nodes-equal-to-sum-of-descendants/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn equal_to_descendants(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::postorder(root).1
    }
    fn postorder(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match node {
            Some(n) => {
                let (left_sum, left_cnt) = Self::postorder(n.borrow().left.clone());
                let (right_sum, right_cnt) = Self::postorder(n.borrow().right.clone());
                let value = n.borrow().val;
                (
                    left_sum + right_sum + value,
                    left_cnt + right_cnt + if value == left_sum + right_sum { 1 } else { 0 },
                )
            }
            None => (0, 0),
        }
    }
}
