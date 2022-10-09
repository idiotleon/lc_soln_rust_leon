use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/two-sum-iv-input-is-a-bst/description/
/// Time Complexity:    O(N)
/// Space Complexity:   O(N)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let sorted: Vec<i32> = {
            let mut sorted: Vec<i32> = Vec::new();
            Self::inorder(root, &mut sorted);
            sorted
        };
        let len_ns: usize = sorted.len();
        let mut lo: isize = 0;
        let mut hi: isize = len_ns as isize - 1;
        while lo < hi {
            let sum = sorted[lo as usize] + sorted[hi as usize];
            if sum == k {
                return true;
            }
            if sum > k {
                hi -= 1;
            } else {
                lo += 1;
            }
        }
        return false;
    }
    fn inorder(node: Option<Rc<RefCell<TreeNode>>>, sorted: &mut Vec<i32>) {
        if let Some(n) = node {
            Self::inorder(n.borrow().left.clone(), sorted);
            sorted.push(n.borrow().val);
            Self::inorder(n.borrow().right.clone(), sorted);
        }
    }
}
