use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// @author: Leon
/// https://leetcode.com/problems/minimum-depth-of-binary-tree/
/// Time Complexity:    O(N)
/// Space Complexity:   O(W)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        queue.push_back(root.unwrap());
        let mut lvl: u16 = 1;
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            for _ in 0..len_q {
                if let Some(cur) = queue.pop_front() {
                    let left = cur.borrow().left.clone();
                    let right = cur.borrow().right.clone();
                    if left.clone().is_none() && right.clone().is_none() {
                        return lvl as i32;
                    }
                    if let Some(l) = left.clone() {
                        queue.push_back(l);
                    }
                    if let Some(r) = right.clone() {
                        queue.push_back(r);
                    }
                }
            }
            lvl += 1;
        }
        unreachable!()
    }
}
