use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/flatten-binary-tree-to-linked-list/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
/// Reference:
/// https://leetcode.com/problems/flatten-binary-tree-to-linked-list/discuss/1208091/Rust-recursive-solution-with-no-clone
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(rt) = root {
            let mut node = rt.borrow_mut();
            Self::flatten(&mut node.left);
            if let Some(left) = node.left.take() {
                let value = left.borrow().val;
                node.right = Some(Rc::new(RefCell::new(TreeNode {
                    val: value,
                    left: left.borrow_mut().right.take(),
                    right: node.right.take(),
                })));
            }
            Self::flatten(&mut node.right);
        }
    }
}
