use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// https://leetcode.com/problems/diameter-of-binary-tree/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
/// Reference:
/// https://leetcode.com/problems/diameter-of-binary-tree/discuss/981923/Rust-DFS
/// https://leetcode.com/problems/diameter-of-binary-tree/discuss/898404/Rust-0ms-100
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (_max_depth, max_diameter) = Self::postorder(root);
        max_diameter
    }
    fn postorder(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match root {
            Some(n) => {
                let (left_max_depth, left_max_diameter) = Self::postorder(n.borrow().left.clone());
                let (right_max_depth, right_max_diameter) =
                    Self::postorder(n.borrow().right.clone());
                let max_depth = std::cmp::max(left_max_depth, right_max_depth) + 1;
                let max_diameter = {
                    let mut tmp = left_max_depth + right_max_depth;
                    tmp = tmp.max(left_max_diameter);
                    tmp = tmp.max(right_max_diameter);
                    tmp
                };
                (max_depth, max_diameter)
            }
            None => (0, 0),
        }
    }
}
