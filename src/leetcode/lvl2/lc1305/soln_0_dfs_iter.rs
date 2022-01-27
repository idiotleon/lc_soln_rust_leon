use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/all-elements-in-two-binary-search-trees/
/// Time Copmlexity:    O(N)
/// Space Complexity:   O(H)
/// Reference:
/// https://leetcode.com/problems/all-elements-in-two-binary-search-trees/discuss/464073/C%2B%2B-One-Pass-Traversal
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_all_elements(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        let mut stk1: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        Self::push_left(root1, &mut stk1);
        let mut stk2: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        Self::push_left(root2, &mut stk2);
        let mut ans: Vec<i32> = Vec::new();
        while !stk1.is_empty() || !stk2.is_empty() {
            let mut stk = if stk1.is_empty() {
                &mut stk2
            } else if stk2.is_empty() {
                &mut stk1
            } else {
                if stk1.back().unwrap().borrow().val > stk2.back().unwrap().borrow().val {
                    &mut stk2
                } else {
                    &mut stk1
                }
            };
            if let Some(node) = stk.pop_back() {
                ans.push(node.borrow().val);
                Self::push_left(node.borrow().right.clone(), &mut stk);
            }
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
