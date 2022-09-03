use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/binary-tree-longest-consecutive-sequence/
/// Time Complexity:    O(N)
/// Space Complexity:   O(W)
/// https://leetcode.com/problems/binary-tree-longest-consecutive-sequence/discuss/74468/Easy-Java-DFS-is-there-better-time-complexity-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_consecutive(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(rt) = root {
            let value: i32 = rt.borrow().val;
            let mut longest: i32 = 0;
            Self::preorder(Some(rt), 1, value + 1, &mut longest);
            return longest;
        } else {
            return 0;
        }
    }
    fn preorder(node: Option<Rc<RefCell<TreeNode>>>, len: i32, expected: i32, longest: &mut i32) {
        if let Some(n) = node {
            let value = n.borrow().val;
            let cur_len = if expected == value { len + 1 } else { 1 };
            *longest = std::cmp::max(*longest, cur_len);
            Self::preorder(n.borrow().left.clone(), cur_len, value + 1, longest);
            Self::preorder(n.borrow().right.clone(), cur_len, value + 1, longest);
        } else {
            return;
        }
    }
}
