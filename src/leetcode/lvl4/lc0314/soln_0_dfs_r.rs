use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/binary-tree-vertical-order-traversal/
/// Time Complexity:    O()
/// Space Complexity:   O()
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn vertical_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut num_to_vrt_order: BTreeMap<i8, BTreeMap<u8, Vec<i32>>> = BTreeMap::new();
        let mut idx_leftmost: i8 = 0;
        let mut idx_rightmost: i8 = 0;
        Self::dfs(
            root,
            0,
            0,
            &mut num_to_vrt_order,
            &mut idx_leftmost,
            &mut idx_rightmost,
        );
        let ans: Vec<Vec<i32>> = {
            let mut ans: Vec<Vec<i32>> = Vec::new();
            for (_idx_vrt, vrt_idx_to_val) in num_to_vrt_order.into_iter() {
                let mut tmp: Vec<i32> = Vec::new();
                vrt_idx_to_val
                    .into_iter()
                    .for_each(|(_idx_lvl, vals)| tmp.extend(vals));
                ans.push(tmp);
            }
            ans
        };
        ans
    }
    fn dfs(
        node: Option<Rc<RefCell<TreeNode>>>,
        idx_lvl: u8,
        idx_vrt: i8,
        num_to_vrt_order: &mut BTreeMap<i8, BTreeMap<u8, Vec<i32>>>,
        idx_leftmost: &mut i8,
        idx_rightmost: &mut i8,
    ) {
        match node {
            Some(n) => {
                num_to_vrt_order
                    .entry(idx_vrt)
                    .or_default()
                    .entry(idx_lvl)
                    .or_default()
                    .push(n.borrow().val);
                *idx_leftmost = std::cmp::min(*idx_leftmost, idx_vrt);
                *idx_rightmost = std::cmp::max(*idx_rightmost, idx_vrt);
                Self::dfs(
                    n.borrow().left.clone(),
                    idx_lvl + 1,
                    idx_vrt - 1,
                    num_to_vrt_order,
                    idx_leftmost,
                    idx_rightmost,
                );
                Self::dfs(
                    n.borrow().right.clone(),
                    idx_lvl + 1,
                    idx_vrt + 1,
                    num_to_vrt_order,
                    idx_leftmost,
                    idx_rightmost,
                );
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
    fn it_works_with_sample_input_1() {
        let vals_tree: Vec<Option<i32>> =
            vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        let root = BinaryTree::new(&vals_tree);
        let actual = Solution::vertical_order(root);
        let expected = vec![vec![9], vec![3, 15], vec![20], vec![7]];
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let vals_tree: Vec<Option<i32>> = vec![
            Some(3),
            Some(9),
            Some(8),
            Some(4),
            Some(0),
            Some(1),
            Some(7),
        ];
        let root = BinaryTree::new(&vals_tree);
        let actual = Solution::vertical_order(root);
        let expected = vec![vec![4], vec![9], vec![3, 0, 1], vec![8], vec![7]];
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_3() {
        let vals_tree = vec![
            Some(3),
            Some(9),
            Some(8),
            Some(4),
            Some(0),
            Some(1),
            Some(7),
            None,
            None,
            None,
            Some(2),
            Some(5),
        ];
        let root = BinaryTree::new(&vals_tree);
        let actual = Solution::vertical_order(root);
        let expected = vec![vec![4], vec![9, 5], vec![3, 0, 1], vec![8, 2], vec![7]];
        assert_eq!(expected, actual);
    }
}
