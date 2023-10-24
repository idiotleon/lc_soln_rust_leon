use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/maximum-average-subtree/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_average_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> f64 {
        let mut largest: f64 = 0.0;
        Self::postorder(root, &mut largest);
        return largest;
    }
    fn postorder(node: Option<Rc<RefCell<TreeNode>>>, largest: &mut f64) -> (u32, u32) {
        if node.is_none() {
            return (0, 0);
        }
        let node = node.unwrap();
        let (left_sum, left_cnt) = Self::postorder(node.borrow().left.clone(), largest);
        let (right_sum, right_cnt) = Self::postorder(node.borrow().right.clone(), largest);
        let sum = left_sum + right_sum + node.borrow().val as u32;
        let cnt = left_cnt + right_cnt + 1;
        let average = (sum as f64) / (cnt as f64);
        *largest = (*largest).max(average);
        return (sum, cnt);
    }
}
