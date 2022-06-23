use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;
/// @author: Leon
/// https://leetcode.com/problems/convert-bst-to-greater-tree/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
/// Reference:
/// https://leetcode.com/problems/convert-bst-to-greater-tree/discuss/100506/Java-Recursive-O(n)-time/104632
/// https://leetcode.com/problems/convert-bst-to-greater-tree/discuss/100506/Java-Recursive-O(n)-time
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut sum: i32 = 0;
        Self::dfs(root.clone(), &mut sum);
        root
    }
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
        if node.is_none() {
            return;
        }
        let node = node.unwrap();
        Self::dfs(node.borrow().right.clone(), sum);
        node.borrow_mut().val += *sum;
        *sum = node.borrow().val;
        Self::dfs(node.borrow().left.clone(), sum);
    }
}
