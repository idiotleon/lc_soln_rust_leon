use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/binary-tree-right-side-view/
/// Time Complexity:    O(N)
/// Space Complexity:   O(W)
/// Reference:
/// https://leetcode.com/problems/binary-tree-right-side-view/discuss/1052300/Rust-BFS
/// https://leetcode.com/problems/binary-tree-right-side-view/discuss/690995/Rust-BFS
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            Some(node) => {
                let mut ans: Vec<i32> = Vec::new();
                let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
                queue.push_back(node);
                while !queue.is_empty() {
                    let len_q = queue.len();
                    for sz in 0..len_q {
                        let first = queue.pop_front().unwrap();
                        if sz == len_q - 1 {
                            ans.push(first.clone().borrow().val);
                        }
                        if let Some(ref left) = first.clone().borrow().left {
                            queue.push_back(left.clone());
                        }
                        if let Some(ref right) = first.clone().borrow().right {
                            queue.push_back(right.clone());
                        }
                    }
                }
                ans
            }
            None => vec![],
        }
    }
}
