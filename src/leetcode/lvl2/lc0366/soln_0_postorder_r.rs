use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/find-leaves-of-binary-tree/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        Self::postorder(root, &mut ans);
        ans
    }
    fn postorder(node: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<Vec<i32>>) -> i32 {
        if let Some(n) = node {
            let left = Self::postorder(n.borrow().left.clone(), res);
            let right = Self::postorder(n.borrow().right.clone(), res);
            let level = 1 + std::cmp::max(left, right);
            if (res.len() as i32) < 1 + level {
                res.push(Vec::new());
            }
            res[level as usize].push(n.borrow().val);
            return level;
        }
        return -1;
    }
}
