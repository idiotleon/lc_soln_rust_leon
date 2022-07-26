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
        let val_p = p.unwrap().borrow().val;
        let val_q = q.unwrap().borrow().val;
        Self::dfs(root, val_p, val_q)
    }
    fn dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        val_p: i32,
        val_q: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(rt) = root {
            let val: i32 = rt.borrow().val;
            if val == val_p || val == val_q {
                return Some(rt);
            }
            let left = Self::dfs(rt.borrow().left.clone(), val_p, val_q);
            let right = Self::dfs(rt.borrow().right.clone(), val_p, val_q);
            return match (left.clone(), right.clone()) {
                (Some(_), Some(_)) => Some(rt),
                (None, None) => None,
                (Some(_), None) => left,
                (None, Some(_)) => right,
            };
        } else {
            return None;
        }
    }
}
