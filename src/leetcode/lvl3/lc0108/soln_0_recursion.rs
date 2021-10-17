use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
////// Reference:
/// https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/discuss/326589/Rust-0ms-3.4MB
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        match nums.len() {
            0 => None,
            _ => Self::build_bst(nums.as_slice()),
        }
    }
    fn build_bst(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        let len_n = nums.len();
        let mid = len_n / 2;
        let node: TreeNode = {
            let mut tmp = TreeNode::new(nums[mid]);
            if mid > 0 {
                tmp.left = Self::build_bst(&nums[..mid]);
            }
            if mid + 1 < len_n {
                tmp.right = Self::build_bst(&nums[mid + 1..]);
            }
            tmp
        };
        Some(Rc::new(RefCell::new(node)))
    }
}
