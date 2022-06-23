use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// @author: Leon
/// https://leetcode.com/problems/binary-tree-level-order-traversal/
/// Time Complexity:    O(N)
/// Space Complexity:   O(W)
/// Reference:
/// https://leetcode.com/problems/binary-tree-level-order-traversal/discuss/1220016/Rust-BFS-solution-with-no-.clone
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::<Rc<RefCell<TreeNode>>>::new();
        if let Some(r) = root {
            queue.push_back(r);
        }
        let mut ans = Vec::<Vec<i32>>::new();
        while !queue.is_empty() {
            let len = queue.len();
            let mut lvl = Vec::<i32>::new();
            for _ in 0..len {
                if let Some(node) = queue.pop_front() {
                    lvl.push(node.borrow().val);
                    if let Some(l) = node.borrow_mut().left.take() {
                        queue.push_back(l);
                    }
                    if let Some(r) = node.borrow_mut().right.take() {
                        queue.push_back(r);
                    }
                }
            }
            ans.push(lvl);
        }
        ans
    }
}
