use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// https://leetcode.com/problems/count-complete-tree-nodes/
/// Time Complexity:    O(H)
/// Space Complexity:   O(H)
/// Reference:
/// https://leetcode.com/problems/count-complete-tree-nodes/discuss/514004/Rust-recursive-solution
#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(r) = root {
            let ld = {
                let mut h = 1;
                let mut node = r.borrow().left.clone();
                while let Some(n) = node {
                    h += 1;
                    node = n.borrow().left.clone();
                }
                h
            };
            let rd = {
                let mut h = 1;
                let mut node = r.borrow().right.clone();
                while let Some(n) = node {
                    h += 1;
                    node = n.borrow().right.clone();
                }
                h
            };
            if ld == rd {
                2i32.pow(ld) - 1
            } else {
                1 + Self::count_nodes(r.borrow().left.clone())
                    + Self::count_nodes(r.borrow().right.clone())
            }
        } else {
            0
        }
    }
}
