use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/minimum-absolute-difference-in-bst/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        const RANGE: i32 = 1e5 as i32 + 7;
        let mut prev: i32 = RANGE;
        let mut min_diff: i32 = RANGE;
        let mut stk: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        Self::push_left(root, &mut stk);
        while !stk.is_empty() {
            if let Some(top) = stk.pop_back() {
                let value = top.borrow().val;
                min_diff = std::cmp::min(min_diff, (prev - value).abs());
                prev = value;
                Self::push_left(top.borrow().right.clone(), &mut stk);
            }
        }
        min_diff
    }
    fn push_left(node: Option<Rc<RefCell<TreeNode>>>, stk: &mut VecDeque<Rc<RefCell<TreeNode>>>) {
        let mut cur = node;
        while let Some(n) = cur {
            stk.push_back(n.clone());
            cur = n.borrow().left.clone();
        }
    }
}
