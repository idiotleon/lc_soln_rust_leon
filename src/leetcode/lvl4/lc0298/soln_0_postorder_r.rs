use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/binary-tree-longest-consecutive-sequence/
/// Time Complexity:    O(N)
/// Space Complexity:   O(W)
/// Reference:
/// https://leetcode.com/problems/binary-tree-longest-consecutive-sequence/discuss/74467/Simple-Recursive-DFS-without-global-variable/140387
/// https://leetcode.com/problems/binary-tree-longest-consecutive-sequence/discuss/74468/Easy-Java-DFS-is-there-better-time-complexity-solution/251091
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_consecutive(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (_len, longest) = Self::postorder(root);
        return longest;
    }
    fn postorder(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(n) = node {
            let (len_left, longest_left) = Self::postorder(n.borrow().left.clone());
            let (len_right, longest_right) = Self::postorder(n.borrow().right.clone());
            let longest = std::cmp::max(longest_left, longest_right);
            let value: i32 = n.borrow().val;
            let cur_len_left: i32 = {
                let mut len: i32 = 1;
                if let Some(l) = n.borrow().left.clone() {
                    if value + 1 == l.borrow().val {
                        len += len_left;
                    }
                }
                len
            };
            let cur_len_right: i32 = {
                let mut len: i32 = 1;
                if let Some(r) = n.borrow().right.clone() {
                    if value + 1 == r.borrow().val {
                        len += len_right;
                    }
                }
                len
            };
            let cur_len = std::cmp::max(cur_len_left, cur_len_right);
            return (cur_len, std::cmp::max(cur_len, longest));
        } else {
            return (0, 0);
        }
    }
}
