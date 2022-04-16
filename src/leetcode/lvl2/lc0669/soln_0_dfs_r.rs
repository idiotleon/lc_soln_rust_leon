use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

/// @author: Leon
/// https://leetcode.com/problems/trim-a-binary-search-tree/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::postorder(root, low, high)
    }

    fn postorder(
        node: Option<Rc<RefCell<TreeNode>>>,
        lo: i32,
        hi: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(n) = node {
            n.clone().borrow_mut().left = Self::postorder(n.borrow().left.clone(), lo, hi);
            n.clone().borrow_mut().right = Self::postorder(n.borrow().right.clone(), lo, hi);
            let value = n.borrow().val;
            if value < lo {
                return Self::postorder(n.borrow().right.clone(), lo, hi);
            }
            if value > hi {
                return Self::postorder(n.borrow().left.clone(), lo, hi);
            }
        }
        None
    }
}
