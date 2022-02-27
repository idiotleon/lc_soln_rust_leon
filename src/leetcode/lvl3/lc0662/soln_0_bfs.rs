use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/maximum-width-of-binary-tree/
/// Time Complexity:    O(N)
/// Space Complexity:   O(W)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue: VecDeque<(Rc<RefCell<TreeNode>>, u32)> = VecDeque::new();
        queue.push_back((root.unwrap(), 1));
        let mut widest: u32 = 0;
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            let mut first: u32 = u32::MAX;
            let mut last: u32 = 0;
            for _ in 0..len_q {
                if let Some((cur, idx_cur)) = queue.pop_front() {
                    first = std::cmp::min(first, idx_cur);
                    last = std::cmp::max(last, idx_cur);
                    if let Some(l) = cur.borrow().left.clone() {
                        queue.push_back((l, idx_cur * 2));
                    }
                    if let Some(r) = cur.borrow().right.clone() {
                        queue.push_back((r, idx_cur * 2 + 1));
                    }
                }
            }
            widest = std::cmp::max(widest, last - first + 1);
        }
        widest as i32
    }
}
