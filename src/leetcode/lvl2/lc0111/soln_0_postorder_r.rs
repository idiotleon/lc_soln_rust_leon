use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// @author: Leon
/// https://leetcode.com/problems/minimum-depth-of-binary-tree/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/minimum-depth-of-binary-tree/discuss/36045/My-4-Line-java-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::postorder(root) as i32
    }
    fn postorder(node: Option<Rc<RefCell<TreeNode>>>) -> u16 {
        match node {
            Some(n) => {
                let left = n.borrow().left.clone();
                let right = n.borrow().right.clone();
                let left_height = Self::postorder(left.clone());
                let right_height = Self::postorder(right.clone());
                if left.clone().is_none() || right.clone().is_none() {
                    left_height + right_height + 1
                } else {
                    1 + std::cmp::min(left_height, right_height)
                }
            }
            None => 0,
        }
    }
}
