/// https://leetcode.com/problems/flatten-binary-tree-to-linked-list/
/// 
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
/// 
/// Reference:
/// https://leetcode.com/problems/flatten-binary-tree-to-linked-list/discuss/1208091/Rust-recursive-solution-with-no-clone
use std::rc::Rc;
use std::cell::RefCell;

use crate::leetcode::util::data_structure::tree_node::TreeNode;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root{
            let mut bm = node.borrow_mut();
            Self::flatten(&mut bm.left);
            if let Some(left) = bm.left.take(){
                let value = left.borrow().val;
                bm.right = Some(Rc::new(RefCell::new(TreeNode{
                    val: value,
                    left: left.borrow_mut().right.take(),
                    right: bm.right.take(),
                })));
            }
            Self::flatten(&mut bm.right);
        }
    }
}