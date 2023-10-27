use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/check-completeness-of-a-binary-tree/
/// Time Complexity:    O(N)
/// Space Compleixty:   O(H)
/// Reference:
/// https://leetcode.com/problems/check-completeness-of-a-binary-tree/editorial/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let total: u16 = Self::get_count(root.clone());
        return Self::dfs(root, 0, total);
    }
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, idx: u16, total: u16) -> bool {
        return if let Some(n) = node {
            if idx >= total {
                return false;
            }
            Self::dfs(n.borrow().left.clone(), idx * 2 + 1, total)
                && Self::dfs(n.borrow().right.clone(), idx * 2 + 2, total)
        } else {
            true
        };
    }
    fn get_count(node: Option<Rc<RefCell<TreeNode>>>) -> u16 {
        return if let Some(n) = node {
            1 + Self::get_count(n.borrow().left.clone()) + Self::get_count(n.borrow().right.clone())
        } else {
            0
        };
    }
}
