use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/maximum-level-sum-of-a-binary-tree/
/// Time Complexity:    O(N)
/// Space Complexity:   O(W)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let root = root.unwrap();
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = {
            let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
            queue.push_back(root);
            queue
        };
        let mut max: i32 = i32::MIN;
        let mut ans: i32 = 1;
        let mut lvl: i32 = 1;
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            let mut sum: i32 = 0;
            for _ in 0..len_q {
                if let Some(cur) = queue.pop_front() {
                    sum += cur.borrow().val;
                    if let Some(l) = cur.borrow().left.clone() {
                        queue.push_back(l);
                    }
                    if let Some(r) = cur.borrow().right.clone() {
                        queue.push_back(r);
                    }
                }
            }
            if max < sum {
                max = sum;
                ans = lvl;
            }
            lvl += 1;
        }
        return ans;
    }
}
