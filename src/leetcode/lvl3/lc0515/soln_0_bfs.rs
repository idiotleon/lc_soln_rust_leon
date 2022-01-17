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
        match root {
            Some(node) => {
                let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
                queue.push_back(node);
                let mut ans: Vec<i32> = Vec::new();
                let mut depth: usize = 1;
                while !queue.is_empty() {
                    let len_q: usize = queue.len();
                    for _ in 0..len_q {
                        if let Some(cur) = queue.pop_front() {
                            if ans.len() < depth {
                                ans.push(cur.borrow().val);
                            } else {
                                ans[depth - 1] = std::cmp::max(ans[depth - 1], cur.borrow().val);
                            }
                            if let Some(l) = cur.borrow().left.clone() {
                                queue.push_back(l);
                            }
                            if let Some(r) = cur.borrow().right.clone() {
                                queue.push_back(r);
                            }
                        }
                    }
                    depth += 1;
                }
                ans
            }
            None => vec![],
        }
    }
}
