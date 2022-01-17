use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;
/// @author: Leon
/// https://leetcode.com/problems/find-largest-value-in-each-tree-row/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        Self::preorder(root, 1, &mut ans);
        ans
    }
    fn preorder(node: Option<Rc<RefCell<TreeNode>>>, lvl: usize, res: &mut Vec<i32>) {
        if let Some(n) = node {
            let val = n.borrow().val;
            if res.len() < lvl {
                res.push(val);
            } else {
                res[lvl - 1] = std::cmp::max(res[lvl - 1], val);
            }
            Self::preorder(n.borrow().left.clone(), lvl + 1, res);
            Self::preorder(n.borrow().right.clone(), lvl + 1, res);
        }
    }
}
