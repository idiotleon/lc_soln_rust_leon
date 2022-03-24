use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
/// @author: Leon
/// https://leetcode.com/problems/binary-search-tree-iterator/
/// Time Complexities:
///     `new`:      O(H)
///     `next`:     O(H)
///     `has_next`: O(1)
/// Space Complexity:   O(H)
#[allow(dead_code)]
struct BSTIterator {
    stk: VecDeque<Rc<RefCell<TreeNode>>>,
}

#[allow(dead_code)]
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let stk: VecDeque<Rc<RefCell<TreeNode>>> = {
            let mut stk: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
            if let Some(rt) = root {
                Self::push_left(&mut stk, rt);
            }
            stk
        };
        BSTIterator { stk }
    }

    fn next(&mut self) -> i32 {
        if let Some(top) = self.stk.pop_back() {
            if let Some(r) = top.borrow().right.clone() {
                Self::push_left(&mut self.stk, r);
            }
            return top.borrow().val;
        }
        unreachable!()
    }

    fn has_next(&self) -> bool {
        !self.stk.is_empty()
    }

    fn push_left(stk: &mut VecDeque<Rc<RefCell<TreeNode>>>, node: Rc<RefCell<TreeNode>>) {
        stk.push_back(node.clone());
        let mut cur = node;
        while let Some(l) = cur.clone().borrow().left.clone() {
            stk.push_back(l.clone());
            cur = l.clone();
        }
    }
}
