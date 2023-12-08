use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/construct-string-from-binary-tree/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut ans: String = String::new();
        Self::dfs(root, &mut ans);
        return ans;
    }
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, res: &mut String) {
        if node.is_none() {
            return;
        }
        let node = node.unwrap();
        res.push_str(&(node.borrow().val.to_string()));
        if node.borrow().left.is_none() && node.borrow().right.is_none() {
            return;
        }
        res.push('(');
        Self::dfs(node.borrow().left.clone(), res);
        res.push(')');
        if !node.borrow().right.is_none() {
            res.push('(');
            Self::dfs(node.borrow().right.clone(), res);
            res.push(')');
        }
    }
}
