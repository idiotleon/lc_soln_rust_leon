use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/insert-into-a-binary-search-tree/
/// Time Complexity:    O(H)
/// Space Complexity:   O(H)
/// Reference:
/// https://leetcode.com/problems/insert-into-a-binary-search-tree/discuss/881890/Rust-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(match root {
            None => Rc::new(RefCell::new(TreeNode::new(val))),
            Some(r) => {
                if r.borrow().val > val {
                    let node = Self::insert_into_bst(r.borrow().left.clone(), val);
                    r.borrow_mut().left = node;
                } else {
                    let node = Self::insert_into_bst(r.borrow().right.clone(), val);
                    r.borrow_mut().right = node;
                }
                r
            }
        })
    }
}
