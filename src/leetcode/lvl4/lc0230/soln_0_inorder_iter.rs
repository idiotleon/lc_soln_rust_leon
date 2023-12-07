use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/kth-smallest-element-in-a-bst/
/// Time Complexity:    O(`k`)
/// Space Complexity:   O(H)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut stk: VecDeque<Rc<RefCell<TreeNode>>> = {
            let mut stk: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
            Self::push_left(root, &mut stk);
            stk
        };
        let mut order: i32 = 0;
        while let Some(top) = stk.pop_back() {
            order += 1;
            if order == k {
                return top.borrow().val;
            }
            Self::push_left(top.borrow().right.clone(), &mut stk);
        }
        unreachable!()
    }
    fn push_left(node: Option<Rc<RefCell<TreeNode>>>, stk: &mut VecDeque<Rc<RefCell<TreeNode>>>) {
        let mut cur = node;
        while let Some(n) = cur {
            stk.push_back(n.clone());
            cur = n.clone().borrow().left.clone();
        }
    }
}
