use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/binary-tree-paths/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();
        Self::preorder(root, &""[..], true, &mut ans);
        return ans;
    }
    fn preorder(
        node: Option<Rc<RefCell<TreeNode>>>,
        path: &str,
        is_root: bool,
        paths: &mut Vec<String>,
    ) {
        match node {
            Some(n) => {
                let val = n.borrow().val;
                let res = if is_root {
                    format!("{}", val)
                } else {
                    format!("{}->{}", path, val)
                };
                if n.borrow().left.is_none() && n.borrow().right.is_none() {
                    paths.push(res.to_owned());
                    return;
                }
                Self::preorder(n.borrow().left.clone(), &res, false, paths);
                Self::preorder(n.borrow().right.clone(), &res, false, paths);
            }
            None => {
                // to do nothing
            }
        }
    }
}
