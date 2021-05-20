/// https://leetcode.com/problems/binary-tree-level-order-traversal/
///
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
///
/// Reference:
/// https://leetcode.com/problems/binary-tree-level-order-traversal/discuss/267006/Rust-0ms-DFS-recursive
use crate::leetcode::util::data_structure::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = Vec::<Vec<i32>>::new();
        Self::dfs(&root, 0, &mut ans);
        ans
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, level: usize, res: &mut Vec<Vec<i32>>) {
        if let Some(n) = node {
            let value = n.borrow().val;
            if level == res.len() {
                res.push(vec![value]);
            } else {
                res[level].push(value);
            }
            Self::dfs(&n.borrow().left, level + 1, res);
            Self::dfs(&n.borrow().right, level + 1, res);
        }
    }
}
