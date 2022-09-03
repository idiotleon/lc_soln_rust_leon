use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/binary-tree-longest-consecutive-sequence/
/// Time Complexity:    O(N)
/// Space Complexity:   O(W)
/// Reference:
/// https://leetcode.com/problems/binary-tree-longest-consecutive-sequence/discuss/761305/Java-or-DFS-and-BFS
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_consecutive(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(rt) = root {
            let mut queue: VecDeque<(Rc<RefCell<TreeNode>>, i32)> = {
                let mut queue: VecDeque<(Rc<RefCell<TreeNode>>, i32)> = VecDeque::new();
                queue.push_back((rt, 1));
                queue
            };
            let mut longest: i32 = 1;
            while !queue.is_empty() {
                let len_q: usize = queue.len();
                for _ in 0..len_q {
                    if let Some((node, len_cur)) = queue.pop_front() {
                        let value: i32 = node.borrow().val;
                        let expected: i32 = value + 1;
                        let len_left: i32 = {
                            let mut len: i32 = 0;
                            if let Some(l) = node.borrow().left.clone() {
                                len = if expected == l.borrow().val {
                                    1 + len_cur
                                } else {
                                    1
                                };
                                queue.push_back((l, len));
                            }
                            len
                        };
                        let len_right: i32 = {
                            let mut len: i32 = 0;
                            if let Some(r) = node.borrow().right.clone() {
                                len = if expected == r.borrow().val {
                                    1 + len_cur
                                } else {
                                    1
                                };
                                queue.push_back((r, len));
                            }
                            len
                        };
                        longest = std::cmp::max(longest, std::cmp::max(len_left, len_right));
                    }
                }
            }
            return longest;
        } else {
            return 0;
        }
    }
}
