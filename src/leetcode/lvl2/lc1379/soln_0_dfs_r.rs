use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/find-a-corresponding-node-of-a-binary-tree-in-a-clone-of-that-tree/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
/// Reference:
/// https://leetcode.com/problems/find-a-corresponding-node-of-a-binary-tree-in-a-clone-of-that-tree/editorial/
struct Solution;

#[allow(dead_code)]
impl Solution {
    fn get_target_copy(
        original: Option<Rc<RefCell<TreeNode>>>,
        cloned: Option<Rc<RefCell<TreeNode>>>,
        target: Rc<RefCell<TreeNode>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut ans: Option<Rc<RefCell<TreeNode>>> = None;
        Self::dfs(original, cloned, target, &mut ans);
        return ans;
    }
    fn dfs(
        original: Option<Rc<RefCell<TreeNode>>>,
        cloned: Option<Rc<RefCell<TreeNode>>>,
        target: Rc<RefCell<TreeNode>>,
        res: &mut Option<Rc<RefCell<TreeNode>>>,
    ) {
        if original.is_none() {
            return;
        }
        let original = original.unwrap();
        if original == target {
            *res = cloned;
            return;
        }
        let cloned = cloned.unwrap();
        Self::dfs(
            original.borrow().left.clone(),
            cloned.borrow().left.clone(),
            target.clone(),
            res,
        );
        Self::dfs(
            original.borrow().right.clone(),
            cloned.borrow().right.clone(),
            target.clone(),
            res,
        );
    }
}
