use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/sum-of-left-leaves/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum: i32 = 0;
        Self::postorder(root, &mut sum);
        return sum;
    }
    fn postorder(node: Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
        if node.is_none() {
            return;
        }
        let node = node.unwrap();
        if let Some(l) = node.borrow().left.clone() {
            if Self::is_leaf(l.clone()) {
                *sum += l.clone().borrow().val;
            }
        }
        Self::postorder(node.borrow().left.clone(), sum);
        Self::postorder(node.borrow().right.clone(), sum);
    }
    fn is_leaf(node: Rc<RefCell<TreeNode>>) -> bool {
        if node.borrow().left.is_none() && node.borrow().right.is_none() {
            return true;
        }
        return false;
    }
}
