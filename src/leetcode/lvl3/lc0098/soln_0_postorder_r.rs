use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/validate-binary-search-tree/
///
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_valid(&root, None, None)
    }
    fn is_valid(
        node: &Option<Rc<RefCell<TreeNode>>>,
        lower_bound: Option<i32>,
        upper_bound: Option<i32>,
    ) -> bool {
        if let Some(n) = node {
            let value = n.borrow().val;
            if let Some(lb) = lower_bound {
                if lb >= value {
                    return false;
                }
            }
            if let Some(ub) = upper_bound {
                if ub <= value {
                    return false;
                }
            }
            return Self::is_valid(&n.borrow().left, lower_bound, Some(value))
                && Self::is_valid(&n.borrow().right, Some(value), upper_bound);
        }
        true
    }
}
