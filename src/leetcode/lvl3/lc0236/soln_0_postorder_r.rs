use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        return Self::postorder(root, p, q);
    }
    fn postorder(
        node: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if node.is_none() || node == p || node == q {
            return node;
        }
        let node = node.unwrap();
        let left = Self::postorder(node.borrow().left.clone(), p.clone(), q.clone());
        let right = Self::postorder(node.borrow().right.clone(), p.clone(), q.clone());
        return if left.is_none() {
            right
        } else if right.is_none() {
            left
        } else {
            Some(node)
        };
    }
}
