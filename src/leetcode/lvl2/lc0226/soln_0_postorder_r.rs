use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;
/// https://leetcode.com/problems/invert-binary-tree/
///
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
///
/// Reference:
/// https://leetcode.com/problems/invert-binary-tree/discuss/648167/Rust-solutions
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(n) = root {
            let left = n.borrow().left.clone();
            let right = n.borrow().right.clone();

            n.borrow_mut().left = Self::invert_tree(right);
            n.borrow_mut().right = Self::invert_tree(left);

            return Some(n);
        }

        None
    }
}
