use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// https://leetcode.com/problems/second-minimum-node-in-a-binary-tree/
/// Time Complexity:    O(N)
/// Space Complexity:   O(W)
struct Solution;

#[allow(dead_code)]
impl Solution {
    const IMPOSSIBLE: i32 = -1;
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
                queue.push_back(node);
                let mut min: i32 = Self::IMPOSSIBLE;
                let mut min_sec: i32 = Self::IMPOSSIBLE;
                while !queue.is_empty() {
                    let len_q: usize = queue.len();
                    for _ in 0..len_q {
                        if let Some(cur) = queue.pop_front() {
                            let value = cur.borrow().val;
                            if min == Self::IMPOSSIBLE || value < min {
                                min_sec = min;
                                min = value;
                            } else if value != min
                                && (min_sec == Self::IMPOSSIBLE || value < min_sec)
                            {
                                min_sec = value;
                            }
                            if let Some(left) = cur.borrow().left.clone() {
                                queue.push_back(left);
                            }
                            if let Some(right) = cur.borrow().right.clone() {
                                queue.push_back(right);
                            }
                        }
                    }
                }
                min_sec
            }
            None => -1,
        }
    }
}
