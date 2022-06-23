use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/maximum-product-of-splitted-binary-tree/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
/// Reference:
/// https://leetcode.com/problems/maximum-product-of-splitted-binary-tree/discuss/496496/Java-Two-Pass-PostOrder-Traversal
/// https://leetcode.com/problems/maximum-product-of-splitted-binary-tree/discuss/496549/JavaC%2B%2BPython-Easy-and-Concise
struct Solution;

#[allow(dead_code)]
impl Solution {
    const MOD: i64 = 1e9 as i64 + 7;
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let sum = Self::get_sum(&root);
        let mut max_product: i64 = 0;
        Self::get_max_product(&root, &mut max_product, sum);
        (max_product % Self::MOD) as i32
    }
    fn get_max_product(
        node: &Option<Rc<RefCell<TreeNode>>>,
        max_product: &mut i64,
        sum: i64,
    ) -> i64 {
        if let Some(n) = node {
            let left = Self::get_max_product(&n.borrow().left, max_product, sum);
            let right = Self::get_max_product(&n.borrow().right, max_product, sum);
            let cur_sum = left + right + n.borrow().val as i64;
            let product = cur_sum * (sum - cur_sum);
            if product > *max_product {
                *max_product = product;
            }
            cur_sum
        } else {
            0
        }
    }
    fn get_sum(node: &Option<Rc<RefCell<TreeNode>>>) -> i64 {
        if let Some(n) = node {
            let left = Self::get_sum(&n.borrow().left);
            let right = Self::get_sum(&n.borrow().right);
            left + right + n.borrow().val as i64
        } else {
            0
        }
    }
}
