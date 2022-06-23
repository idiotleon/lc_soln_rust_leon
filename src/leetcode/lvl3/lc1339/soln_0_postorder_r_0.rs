use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/maximum-product-of-splitted-binary-tree/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/maximum-product-of-splitted-binary-tree/discuss/1413525/Rust-DFS-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    const MOD: i64 = 1e9 as i64 + 7;
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sums: Vec<i64> = Vec::new();
        Self::postorder(&root, &mut sums);
        ((0..sums.len() - 1)
            .map(|i| sums[i] * (sums[sums.len() - 1] - sums[i]))
            .max()
            .unwrap()
            % Self::MOD) as i32
    }
    fn postorder(node: &Option<Rc<RefCell<TreeNode>>>, sums: &mut Vec<i64>) -> i64 {
        if let Some(n) = node {
            let sum = n.borrow().val as i64
                + Self::postorder(&n.borrow().left, sums)
                + Self::postorder(&n.borrow().right, sums);
            sums.push(sum);
            sum
        } else {
            0
        }
    }
}
