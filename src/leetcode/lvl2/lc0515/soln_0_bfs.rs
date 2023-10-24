use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/find-largest-value-in-each-tree-row/
/// Time Complexity:    O(N)
/// Space Complexity:   O(W)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        if root.is_none() {
            return ans;
        }
        let root = root.unwrap();
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = {
            let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
            queue.push_back(root);
            queue
        };
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            let mut max: i32 = i32::MIN;
            for _ in 0..len_q {
                if let Some(top) = queue.pop_front() {
                    max = std::cmp::max(max, top.borrow().val);
                    if let Some(l) = top.borrow().left.clone() {
                        queue.push_back(l);
                    }
                    if let Some(r) = top.borrow().right.clone() {
                        queue.push_back(r);
                    }
                }
            }
            ans.push(max);
        }
        return ans;
    }
}
