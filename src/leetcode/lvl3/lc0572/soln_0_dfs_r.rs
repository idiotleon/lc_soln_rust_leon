use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/subtree-of-another-tree/
/// Time Complexity:    O(M * N)
/// Space Complexity:   O(M + N)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if root.is_none() {
            return false;
        }
        if Self::dfs(root.clone(), sub_root.clone()) {
            return true;
        }
        let root = root.unwrap();
        return Self::is_subtree(root.borrow().left.clone(), sub_root.clone())
            || Self::is_subtree(root.borrow().right.clone(), sub_root.clone());
    }
    fn dfs(node1: Option<Rc<RefCell<TreeNode>>>, node2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if node1.is_none() || node2.is_none() {
            return node1.is_none() && node2.is_none();
        }
        let node1 = node1.unwrap();
        let node2 = node2.unwrap();
        return node1.borrow().val == node2.borrow().val
            && Self::dfs(node1.borrow().left.clone(), node2.borrow().left.clone())
            && Self::dfs(node1.borrow().right.clone(), node2.borrow().right.clone());
    }
}
