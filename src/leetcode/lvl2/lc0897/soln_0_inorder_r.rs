use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;
type Node = Option<Rc<RefCell<TreeNode>>>;

/// @author: Leon
/// https://leetcode.com/problems/increasing-order-search-tree/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/increasing-order-search-tree/solutions/1958500/rust-0ms-2-2-mb/
/// https://leetcode.com/problems/increasing-order-search-tree/solutions/3325655/rust-implementation/
/// https://leetcode.com/problems/increasing-order-search-tree/solutions/1957652/rust-no-clone-no-additional-data-structures/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        return Self::inorder(root.clone(), None);
    }
    fn inorder(node: Node, tail: Node) -> Node {
        return if let Some(n) = node.clone() {
            let mut bm = n.borrow_mut();
            let res = Self::inorder(bm.left.take(), node);
            bm.right = Self::inorder(bm.right.take(), tail);
            res
        } else {
            tail
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::leetcode::util::data_structure::tree::binary::binary_tree::BinaryTree;
    #[test]
    fn it_works_with_sample_input_1() {
        let nodes: Vec<Option<i32>> = vec![
            Some(5),
            Some(3),
            Some(6),
            Some(2),
            Some(4),
            None,
            Some(8),
            Some(1),
            None,
            None,
            None,
            Some(7),
            Some(9),
        ];
        let root: Option<Rc<RefCell<TreeNode>>> = BinaryTree::new(&nodes);
        let nodes_expected: Vec<Option<i32>> = vec![
            Some(1),
            None,
            Some(2),
            None,
            Some(3),
            None,
            Some(4),
            None,
            Some(5),
            None,
            Some(6),
            None,
            Some(7),
            None,
            Some(8),
            None,
            Some(9),
        ];
        let expected: Option<Rc<RefCell<TreeNode>>> = BinaryTree::new(&nodes_expected);
        let actual: Option<Rc<RefCell<TreeNode>>> = Solution::increasing_bst(root);
        assert!(BinaryTree::is_same_tree(expected, actual));
    }
}
