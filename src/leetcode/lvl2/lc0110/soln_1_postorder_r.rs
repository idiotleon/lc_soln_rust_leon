use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// @author: Leon
/// https://leetcode.com/problems/balanced-binary-tree/
/// Time Complexity:    O(N * lg(N)) ~ O(N ^ 2)
/// Space Complexity:   O(H)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(rt) = root {
            let left = rt.borrow().left.clone();
            let right = rt.borrow().right.clone();
            let left_height = Self::get_height(left.clone());
            let right_height = Self::get_height(right.clone());
            if (left_height - right_height).abs() > 1 {
                return false;
            }
            Self::is_balanced(left.clone()) && Self::is_balanced(right.clone())
        } else {
            true
        }
    }
    fn get_height(node: Option<Rc<RefCell<TreeNode>>>) -> i16 {
        if let Some(n) = node {
            let left_height = Self::get_height(n.borrow().left.clone());
            let right_height = Self::get_height(n.borrow().right.clone());
            1 + std::cmp::max(left_height, right_height)
        } else {
            0
        }
    }
}
