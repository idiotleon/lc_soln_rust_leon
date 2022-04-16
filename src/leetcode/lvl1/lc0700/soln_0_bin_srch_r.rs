use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/search-in-a-binary-search-tree/
/// Time Complexity:    O(H)
/// Space Complexity:   O(H)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        let value: i32 = root.clone().unwrap().borrow().val;
        if value > val {
            return Self::search_bst(root.clone().unwrap().borrow().left.clone(), val);
        } else if value < val {
            return Self::search_bst(root.clone().unwrap().borrow().right.clone(), val);
        } else {
            return root;
        }
    }
}
