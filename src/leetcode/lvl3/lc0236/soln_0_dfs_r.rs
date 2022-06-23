use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
/// Reference:
/// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/discuss/963346/Rust-recursive
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let root = match root {
            Some(a) => a,
            None => return None,
        };
        let val_p = p.unwrap().borrow().val;
        let val_q = q.unwrap().borrow().val;
        Self::dfs(root, val_p, val_q)
    }
    fn dfs(root: Rc<RefCell<TreeNode>>, val_p: i32, val_q: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let val_root = root.borrow().val;
        if val_root == val_p || val_root == val_q {
            return Some(root);
        }
        let left = root
            .borrow()
            .left
            .as_ref()
            .and_then(|a| Self::dfs(a.clone(), val_p, val_q));
        let right = root
            .borrow()
            .right
            .as_ref()
            .and_then(|a| Self::dfs(a.clone(), val_p, val_q));
        match (left, right) {
            (Some(_), Some(_)) => Some(root),
            (None, None) => None,
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
        }
    }
}
