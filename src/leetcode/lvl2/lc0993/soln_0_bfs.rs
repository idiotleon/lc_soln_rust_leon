use std::cell::RefCell;
use std::collections::VecDeque;

use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// https://leetcode.com/problems/cousins-in-binary-tree/
///
/// Time Complexity:    O(N)
/// Space Complexity:   O(W)
///
/// Reference:
/// https://leetcode.com/problems/cousins-in-binary-tree/discuss/1527316/Rust-BFS-solution
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let mut queue = VecDeque::new();
        if let Some(r) = root {
            queue.push_back((r, None));
        }
        while !queue.is_empty() {
            let mut x_found = None;
            let mut y_found = None;
            let size = queue.len();
            for _ in 0..size {
                if let Some((node, parent)) = queue.pop_front() {
                    let val = node.borrow().val;
                    if val == x {
                        x_found = parent;
                    }
                    if val == y {
                        y_found = parent;
                    }
                    if let Some(n) = node.borrow_mut().left.take() {
                        queue.push_back((n, Some(val)));
                    }
                    if let Some(n) = node.borrow_mut().right.take() {
                        queue.push_back((n, Some(val)));
                    }
                }
            }
            if let (Some(x_parent), Some(y_parent)) = (x_found, y_found) {
                return x_parent != y_parent;
            }
        }
        false
    }
}
