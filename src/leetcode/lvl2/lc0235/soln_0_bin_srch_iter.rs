use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/
/// Time Complexity:    O(H)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(rt) = root {
            let mut cur = rt;
            let val_p: i32 = p.unwrap().borrow().val;
            let val_q: i32 = q.unwrap().borrow().val;
            loop {
                let val: i32 = cur.borrow().val;
                cur = if val < val_p && val < val_q {
                    cur.borrow().right.clone().unwrap()
                } else if val > val_p && val > val_q {
                    cur.borrow().left.clone().unwrap()
                } else {
                    return Some(cur);
                }
            }
        }
        unreachable!();
    }
}
