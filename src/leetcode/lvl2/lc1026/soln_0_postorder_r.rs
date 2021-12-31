use std::cell::RefCell;
use std::cmp::{max, min};
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/maximum-difference-between-node-and-ancestor/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::postorder(root).2.unwrap()
    }
    fn postorder(node: Option<Rc<RefCell<TreeNode>>>) -> (Option<i32>, Option<i32>, Option<i32>) {
        match node {
            Some(n) => {
                let (left_min, left_max, left_max_diff) = Self::postorder(n.borrow().left.clone());
                let (right_min, right_max, right_max_diff) =
                    Self::postorder(n.borrow().right.clone());
                let value = n.borrow().val;
                let min_val = min(
                    left_min.unwrap_or(value),
                    min(right_min.unwrap_or(value), value),
                );
                let max_val = max(
                    left_max.unwrap_or(value),
                    max(right_max.unwrap_or(value), value),
                );
                let max_diff = max(
                    (value - min_val).abs(),
                    max(
                        (value - max_val).abs(),
                        max(left_max_diff.unwrap_or(0), right_max_diff.unwrap_or(0)),
                    ),
                );
                (Some(min_val), Some(max_val), Some(max_diff))
            }
            None => (None, None, None),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::leetcode::util::data_structure::tree::binary::binary_tree::BinaryTree;

    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let nodes = vec![
            Some(8),
            Some(3),
            Some(10),
            Some(1),
            Some(6),
            None,
            Some(14),
            None,
            None,
            Some(4),
            Some(7),
            Some(13),
        ];
        let root = BinaryTree::new(&nodes);
        let actual = Solution::max_ancestor_diff(root);
        let expected = 7;
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_test_case_26() {
        let nodes = vec![
            Some(7),
            Some(5),
            Some(12),
            Some(11),
            Some(0),
            Some(2),
            None,
            Some(4),
            Some(17),
            Some(6),
            Some(19),
            None,
            Some(16),
            Some(18),
            None,
            None,
            None,
            None,
            Some(9),
            Some(14),
            Some(10),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(3),
            Some(1),
            None,
            None,
            Some(8),
            None,
            Some(13),
            None,
            None,
            Some(15),
        ];
        let root = BinaryTree::new(&nodes);
        let actual = Solution::max_ancestor_diff(root);
        let expected = 19;
        assert_eq!(expected, actual);
    }
}
