use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/all-elements-in-two-binary-search-trees/
/// Time Complexity:    O(N)
/// Space Complexity:   O(N)
/// Reference:
/// https://leetcode.com/problems/all-elements-in-two-binary-search-trees/discuss/463852/JavaPython-3-2-codes%3A-O(n)-time-w-analysis.
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_all_elements(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        let mut queue1: VecDeque<i32> = VecDeque::new();
        Self::inorder(root1, &mut queue1);
        let len_q1: usize = queue1.len();
        let mut queue2: VecDeque<i32> = VecDeque::new();
        Self::inorder(root2, &mut queue2);
        let len_q2: usize = queue2.len();
        let mut ans: Vec<i32> = Vec::with_capacity(len_q1 + len_q2);
        while !queue1.is_empty() || !queue2.is_empty() {
            if queue1.is_empty() {
                ans.push(queue2.pop_front().unwrap());
            } else if queue2.is_empty() {
                ans.push(queue1.pop_front().unwrap());
            } else {
                ans.push(if queue1.front() > queue2.front() {
                    queue2.pop_front().unwrap()
                } else {
                    queue1.pop_front().unwrap()
                });
            }
        }
        ans
    }
    fn inorder(node: Option<Rc<RefCell<TreeNode>>>, queue: &mut VecDeque<i32>) {
        if let Some(n) = node {
            Self::inorder(n.borrow().left.clone(), queue);
            queue.push_back(n.borrow().val);
            Self::inorder(n.borrow().right.clone(), queue);
        }
    }
}
