use std::{cell::RefCell, rc::Rc};

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// https://leetcode.com/problems/second-minimum-node-in-a-binary-tree/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
struct Solution;

#[allow(dead_code)]
impl Solution {
    const IMPOSSIBLE: i32 = -1;
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(root)
    }
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                if node.borrow().left.is_none(){
                    return -1;
                }
                let left = if node.borrow().val == node.borrow().left.clone().unwrap().borrow().val {
                    Self::dfs(node.borrow().left.clone())
                } else {
                    node.borrow().left.clone().unwrap().borrow().val
                };
                let right = if node.borrow().val == node.borrow().right.clone().unwrap().borrow().val {
                    Self::dfs(node.borrow().right.clone())
                } else {
                    node.borrow().right.clone().unwrap().borrow().val
                };
                if left == Self::IMPOSSIBLE || right == Self::IMPOSSIBLE {
                    std::cmp::max(left, right)
                } else {
                    std::cmp::min(left, right)
                }
            }
            None => Self::IMPOSSIBLE,
        }
    }
}
