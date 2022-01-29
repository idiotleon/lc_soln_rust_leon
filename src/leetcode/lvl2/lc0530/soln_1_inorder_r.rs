use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/minimum-absolute-difference-in-bst/
/// Time Complexity:    O(N)
/// Space Complexity:   O(N)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        const RANGE: i32 = 1e5 as i32 + 7;
        let mut sorted: Vec<i32> = Vec::new();
        Self::inorder(root, &mut sorted);
        let len_s: usize = sorted.len();
        let mut min_diff: i32 = RANGE;
        for idx in 1..len_s {
            min_diff = std::cmp::min(min_diff, sorted[idx] - sorted[idx - 1]);
        }
        min_diff
    }
    fn inorder(node: Option<Rc<RefCell<TreeNode>>>, sorted: &mut Vec<i32>) {
        if let Some(n) = node {
            Self::inorder(n.borrow().left.clone(), sorted);
            sorted.push(n.borrow().val);
            Self::inorder(n.borrow().right.clone(), sorted);
        }
    }
}
