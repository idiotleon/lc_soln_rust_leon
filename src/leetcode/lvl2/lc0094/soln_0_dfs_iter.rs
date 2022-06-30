use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/binary-tree-inorder-traversal/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        let mut stk: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        Self::push_left(root, &mut stk);
        while let Some(top) = stk.pop_back() {
            ans.push(top.borrow().val);
            Self::push_left(top.borrow().right.clone(), &mut stk);
        }
        ans
    }
    fn push_left(node: Option<Rc<RefCell<TreeNode>>>, stk: &mut VecDeque<Rc<RefCell<TreeNode>>>) {
        let mut cur = node;
        while let Some(n) = cur {
            stk.push_back(n.clone());
            cur = n.borrow().left.clone();
        }
    }
}
