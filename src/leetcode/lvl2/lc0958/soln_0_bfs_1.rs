use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/check-completeness-of-a-binary-tree/
/// Time Complexity:    O(N)
/// Space Complexity:   O(W)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }
        let root = root.unwrap();
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        queue.push_back(root);
        let mut null_found: bool = false;
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            for _ in 0..len_q {
                if let Some(cur) = queue.pop_front() {
                    if let Some(l) = cur.borrow().left.clone() {
                        queue.push_back(l);
                    } else {
                        if null_found {
                            return false;
                        }
                        null_found = true;
                    }
                    if let Some(r) = cur.borrow().right.clone() {
                        queue.push_back(r);
                    } else {
                        if null_found {
                            return false;
                        }
                        null_found = true;
                    }
                }
            }
        }
        return true;
    }
}
