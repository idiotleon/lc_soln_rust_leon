use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/path-sum-ii/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut path: Vec<i32> = Vec::new();
        let mut paths: Vec<Vec<i32>> = Vec::new();
        Self::backtrack(root, target_sum, &mut path, &mut paths);
        return paths;
    }
    fn backtrack(
        node: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
        path: &mut Vec<i32>,
        paths: &mut Vec<Vec<i32>>,
    ) {
        if let Some(n) = node {
            let value = n.borrow().val;
            path.push(value);
            if target == value && n.borrow().left.is_none() && n.borrow().right.is_none() {
                paths.push(path.to_vec());
            } else {
                Self::backtrack(n.borrow().left.clone(), target - value, path, paths);
                Self::backtrack(n.borrow().right.clone(), target - value, path, paths);
            }
            path.pop();
        }
    }
}
