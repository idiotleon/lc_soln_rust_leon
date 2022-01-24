use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/univalued-binary-tree/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::dfs(root.clone(), root.unwrap().borrow().val)
    }
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, value: i32) -> bool {
        if node.is_none() {
            return true;
        }
        let node = node.unwrap();
        let val = node.borrow().val;
        if val != value {
            return false;
        }
        return Self::dfs(node.borrow().left.clone(), value)
            && Self::dfs(node.borrow().right.clone(), value);
    }
}
