use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/same-tree/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Self::dfs(p, q)
    }
    fn dfs(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        }
        if p.is_none() || q.is_none() {
            return false;
        }
        let p = p.unwrap();
        let q = q.unwrap();
        if p.borrow().val != q.borrow().val {
            return false;
        }
        let is_same_tree = Self::dfs(p.borrow().left.clone(), q.borrow().left.clone())
            && Self::dfs(p.borrow().right.clone(), q.borrow().right.clone());
        is_same_tree
    }
}
