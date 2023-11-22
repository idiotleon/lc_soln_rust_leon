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
        let mut ans: Vec<i32> = Vec::new();
        if let Some(rt) = root {
            let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
            queue.push_back(rt);
            while !queue.is_empty() {
                let len_q = queue.len();
                for idx in 0..len_q {
                    if let Some(cur) = queue.pop_front() {
                        if idx == len_q - 1 {
                            ans.push(cur.borrow().val);
                        }
                        if let Some(l) = cur.borrow().left.clone() {
                            queue.push_back(l);
                        }
                        if let Some(r) = cur.borrow().right.clone() {
                            queue.push_back(r);
                        }
                    }
                }
            }
        }
        return ans;
    }
}
