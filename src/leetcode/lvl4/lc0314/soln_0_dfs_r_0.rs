use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/binary-tree-vertical-order-traversal/
/// Time Complexity:    O(N)
/// Space Complexity:   O(N)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn vertical_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let coord_to_nodes: BTreeMap<i32, BTreeMap<i32, Vec<i32>>> = {
            let mut map: BTreeMap<i32, BTreeMap<i32, Vec<i32>>> = BTreeMap::new();
            Self::dfs(0, 0, root, &mut map);
            map
        };
        return coord_to_nodes
            .into_values()
            .collect::<Vec<BTreeMap<i32, Vec<i32>>>>()
            .into_iter()
            .map(|map: BTreeMap<i32, Vec<i32>>| {
                map.into_values()
                    .collect::<Vec<Vec<i32>>>()
                    .into_iter()
                    .flatten()
                    .collect()
            })
            .collect();
    }
    fn dfs(
        x: i32,
        y: i32,
        node: Option<Rc<RefCell<TreeNode>>>,
        coord_to_nodes: &mut BTreeMap<i32, BTreeMap<i32, Vec<i32>>>,
    ) {
        if node.is_none() {
            return;
        }
        let node = node.unwrap();
        coord_to_nodes
            .entry(x)
            .or_default()
            .entry(y)
            .or_default()
            .push(node.borrow().val);
        Self::dfs(x - 1, y + 1, node.borrow().left.clone(), coord_to_nodes);
        Self::dfs(x + 1, y + 1, node.borrow().right.clone(), coord_to_nodes);
    }
}
