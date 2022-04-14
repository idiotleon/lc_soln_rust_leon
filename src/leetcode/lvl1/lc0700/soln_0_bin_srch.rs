use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/search-in-a-binary-search-tree/
/// Time Complexity:    O(H)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut cur = root;
        while let Some(node) = cur.clone() {
            let value = node.clone().borrow().val;
            if value < val {
                cur = node.borrow().right.clone();
            } else if value > val {
                cur = node.borrow().left.clone();
            } else {
                return cur;
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use crate::leetcode::util::data_structure::tree::binary::binary_tree::BinaryTree;

    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let root = vec![Some(4), Some(2), Some(7), Some(1), Some(3)];
        let val: i32 = 2;
        let root: Option<Rc<RefCell<TreeNode>>> = BinaryTree::new(&root);
        let actual = Solution::search_bst(root, val);
        let expected = BinaryTree::new(&vec![Some(2), Some(1), Some(3)]);
        assert!(BinaryTree::is_same_tree(expected, actual));
    }
}
