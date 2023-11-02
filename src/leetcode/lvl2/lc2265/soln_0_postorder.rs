use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/count-nodes-equal-to-average-of-subtree/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut cnt: i32 = 0;
        Self::postorder(root, &mut cnt);
        return cnt;
    }
    fn postorder(node: Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> (i32, i32) {
        if node.is_none() {
            return (0, 0);
        }
        let node = node.unwrap();
        let (left_sum, left_cnt) = Self::postorder(node.borrow().left.clone(), res);
        let (right_sum, right_cnt) = Self::postorder(node.borrow().right.clone(), res);
        let cur_val = node.borrow().val;
        let sum = left_sum + right_sum + cur_val;
        let cnt = left_cnt + right_cnt + 1;
        let average = sum / cnt;
        if average == cur_val {
            *res += 1;
        }
        return (sum, cnt);
    }
}
