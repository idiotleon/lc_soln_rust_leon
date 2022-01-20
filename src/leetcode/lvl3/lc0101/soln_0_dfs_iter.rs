use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/symmetric-tree/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
/// Reference:
/// https://leetcode.com/problems/symmetric-tree/discuss/33054/Recursive-and-non-recursive-solutions-in-Java/31849
/// https://leetcode.com/problems/symmetric-tree/discuss/33054/Recursive-and-non-recursive-solutions-in-Java
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(r) => {
                let mut stk: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
                stk.push_back(r.borrow().left.clone());
                stk.push_back(r.borrow().right.clone());
                while !stk.is_empty() {
                    let p = stk.pop_back().unwrap();
                    let q = stk.pop_back().unwrap();
                    if p.is_none() && q.is_none() {
                        continue;
                    }
                    if p.is_none() || q.is_none() {
                        return false;
                    }
                    let p = p.unwrap();
                    let q = q.unwrap();
                    if p.borrow().val != q.borrow().val {
                        return false;
                    }
                    stk.push_back(p.borrow().left.clone());
                    stk.push_back(q.borrow().right.clone());
                    stk.push_back(p.borrow().right.clone());
                    stk.push_back(q.borrow().left.clone());
                }
                true
            }
            None => true,
        }
    }
}
