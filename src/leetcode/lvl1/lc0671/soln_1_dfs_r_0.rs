use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// https://leetcode.com/problems/second-minimum-node-in-a-binary-tree/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    const IMPOSSIBLE: i32 = -1;
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut mins: (i32, i32) = (-1, -1);
        Self::dfs(root, &mut mins);
        let (_, min_sec) = mins;
        min_sec
    }
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, mins: &mut (i32, i32)) {
        match root {
            Some(node) => {
                let value = node.borrow().val;
                let (min, min_sec) = mins;
                if *min == Self::IMPOSSIBLE || value < *min {
                    *min_sec = *min;
                    *min = value;
                } else if value != *min && (*min_sec == Self::IMPOSSIBLE || value < *min_sec) {
                    *min_sec = value;
                }
                Self::dfs(node.borrow().left.clone(), mins);
                Self::dfs(node.borrow().right.clone(), mins);
            }
            None => return,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::leetcode::util::data_structure::tree::binary::binary_tree::BinaryTree;

    #[test]
    fn test_sample_input_1() {
        let nums_op: Vec<Option<i32>> =
            vec![Some(2), Some(2), Some(5), None, None, Some(5), Some(7)];
        let root = BinaryTree::new(&nums_op);
        let actual: i32 = Solution::find_second_minimum_value(root);
        let expected: i32 = 5;
        assert_eq!(expected, actual);
    }
}
