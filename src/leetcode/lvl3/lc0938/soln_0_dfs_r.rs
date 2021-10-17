use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// https://leetcode.com/problems/range-sum-of-bst/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
/// Reference:
/// https://leetcode.com/problems/range-sum-of-bst/discuss/213098/Rust-recursive-solution
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut sum = 0;
        if let Some(node) = root {
            let value = node.borrow().val;
            if low <= value && value <= high {
                sum += value;
            }
            if low <= value {
                sum += Self::range_sum_bst(node.borrow().left.clone(), low, high);
            }
            if value <= high {
                sum += Self::range_sum_bst(node.borrow().right.clone(), low, high);
            }
        }
        sum
    }
}
