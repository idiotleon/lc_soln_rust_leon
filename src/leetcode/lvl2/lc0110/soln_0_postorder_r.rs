use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// @author: Leon
/// https://leetcode.com/problems/balanced-binary-tree/
/// Time Complexity:    O(N * lg(N)) ~ O(N ^ 2)
/// Space Complexity:   O(H)
/// Reference:
/// https://leetcode.com/problems/balanced-binary-tree/discuss/35691/The-bottom-up-O(N)-solution-would-be-better/198436
/// https://leetcode.com/problems/balanced-binary-tree/discuss/35691/The-bottom-up-O(N)-solution-would-be-better
struct Solution;

#[allow(dead_code)]
impl Solution {
    const IMPL: i16 = -1;
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }
        Self::postorder(root) != Self::IMPL
    }
    fn postorder(node: Option<Rc<RefCell<TreeNode>>>) -> i16 {
        match node {
            Some(n) => {
                let left = Self::postorder(n.borrow().left.clone());
                let right = Self::postorder(n.borrow().right.clone());
                if left == Self::IMPL || right == Self::IMPL || (left - right).abs() > 1 {
                    return Self::IMPL;
                }
                std::cmp::max(left, right) + 1
            }
            None => 0,
        }
    }
}
