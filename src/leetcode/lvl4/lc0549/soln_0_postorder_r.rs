use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/binary-tree-longest-consecutive-sequence-ii/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
/// https://leetcode.com/problems/binary-tree-longest-consecutive-sequence-ii/discuss/101519/Neat-Java-Solution-Single-pass-O(n)/105240
/// https://leetcode.com/problems/binary-tree-longest-consecutive-sequence-ii/discuss/101519/Neat-Java-Solution-Single-pass-O(n)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_consecutive(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (_inc, _dec, longest) = Self::postorder(root);
        return longest;
    }
    fn postorder(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
        if let Some(n) = node {
            let (left_inc, left_dec, left_longest) = Self::postorder(n.borrow().left.clone());
            let (right_inc, right_dec, right_longest) = Self::postorder(n.borrow().right.clone());
            let value = n.borrow().val;
            let mut len_inc: i32 = 1;
            let mut len_dec: i32 = 1;
            if let Some(l) = n.borrow().left.clone() {
                let left_val: i32 = l.borrow().val;
                if value - left_val == 1 {
                    len_dec = 1 + left_dec;
                } else if value - left_val == -1 {
                    len_inc = 1 + left_inc;
                }
            }
            if let Some(r) = n.borrow().right.clone() {
                let right_val: i32 = r.borrow().val;
                if value - right_val == 1 {
                    len_dec = std::cmp::max(len_dec, 1 + right_dec);
                } else if value - right_val == -1 {
                    len_inc = std::cmp::max(len_inc, 1 + right_inc);
                }
            }
            let longest = std::cmp::max(
                left_longest,
                std::cmp::max(right_longest, len_inc + len_dec - 1),
            );
            return (len_inc, len_dec, longest);
        } else {
            return (0, 0, 0);
        }
    }
}
