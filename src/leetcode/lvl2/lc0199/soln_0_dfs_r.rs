use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/binary-tree-right-side-view/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
/// Reference:
/// https://leetcode.com/problems/binary-tree-right-side-view/discuss/335805/Rust-Solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        Self::dfs(root, 0, &mut ans);
        ans
    }
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, level: usize, res: &mut Vec<i32>) {
        if let Some(n) = node {
            if level == res.len() {
                res.push(n.borrow().val);
            }
            Self::dfs(n.borrow().right.clone(), 1 + level, res);
            Self::dfs(n.borrow().left.clone(), 1 + level, res);
        }
    }
}
