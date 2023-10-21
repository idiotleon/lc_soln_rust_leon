use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/
/// Time Complexity:    O(N)
/// Space Complexity:   O(W)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return Vec::new();
        }
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let mut lvl: u16 = 0;
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        queue.push_back(root.unwrap());
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            let mut res: Vec<i32> = Vec::new();
            for _ in 0..len_q {
                if let Some(cur) = queue.pop_front() {
                    res.push(cur.borrow().val);
                    if let Some(l) = cur.borrow().left.clone() {
                        queue.push_back(l);
                    }
                    if let Some(r) = cur.borrow().right.clone() {
                        queue.push_back(r);
                    }
                }
            }
            if lvl % 2 == 1 {
                res.reverse();
            }
            lvl += 1;
            ans.push(res);
        }
        return ans;
    }
}
