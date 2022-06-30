use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/binary-tree-preorder-traversal/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        if let Some(rt) = root {
            let mut stk: VecDeque<Rc<RefCell<TreeNode>>> = {
                let mut stk = VecDeque::new();
                stk.push_back(rt);
                stk
            };
            while let Some(top) = stk.pop_back() {
                ans.push(top.borrow().val);
                if let Some(r) = top.borrow().right.clone() {
                    stk.push_back(r);
                }
                if let Some(l) = top.borrow().left.clone() {
                    stk.push_back(l);
                }
            }
            return ans;
        }
        ans
    }
}
