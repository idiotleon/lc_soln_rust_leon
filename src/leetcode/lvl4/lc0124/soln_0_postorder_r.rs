use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/binary-tree-maximum-path-sum/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
struct Solution;

#[allow(dead_code)]
impl Solution {
    const RANGE: i32 = 1e3 as i32 + 1;
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (max_sum, _path_sum) = Self::postorder(root);
        return max_sum;
    }
    fn postorder(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(n) = node {
            let (left_max_sum, left_path_sum) = Self::postorder(n.borrow().left.clone());
            let (right_max_sum, right_path_sum) = Self::postorder(n.borrow().right.clone());
            let value = n.borrow().val;
            let sum = left_path_sum + right_path_sum + value;
            return (
                max(max(left_max_sum, right_max_sum), sum),
                max(0, value + max(left_path_sum, right_path_sum)),
            );
        } else {
            return (-Self::RANGE, 0);
        }
    }
}
