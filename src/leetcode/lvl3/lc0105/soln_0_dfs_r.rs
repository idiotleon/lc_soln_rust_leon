use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let len_ns: usize = preorder.len();
        let inorder_val_to_idx: HashMap<i32, isize> = {
            let mut map: HashMap<i32, isize> = HashMap::with_capacity(len_ns);
            for (idx, &num) in inorder.iter().enumerate() {
                map.insert(num, idx as isize);
            }
            map
        };
        Self::dfs(
            0,
            0,
            len_ns as isize - 1,
            &preorder,
            &inorder,
            &inorder_val_to_idx,
        )
    }
    fn dfs(
        idx_preorder: isize,
        idx_start_inorder: isize,
        idx_end_inorder: isize,
        preorder: &Vec<i32>,
        inorder: &Vec<i32>,
        inorder_val_to_idx: &HashMap<i32, isize>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let len_ns: isize = preorder.len() as isize;
        if idx_preorder >= len_ns || idx_start_inorder > idx_end_inorder {
            return None;
        }
        let root_val: i32 = preorder[idx_preorder as usize];
        let root = Rc::new(RefCell::new(TreeNode::new(root_val)));
        let idx_root_inorder: isize = *inorder_val_to_idx.get(&root_val).unwrap();
        let size_left_subtree: isize = idx_root_inorder - idx_start_inorder;
        root.clone().borrow_mut().left = Self::dfs(
            1 + idx_preorder,
            idx_start_inorder,
            idx_root_inorder - 1,
            preorder,
            inorder,
            inorder_val_to_idx,
        );
        root.clone().borrow_mut().right = Self::dfs(
            idx_preorder + size_left_subtree + 1,
            idx_root_inorder + 1,
            idx_end_inorder,
            preorder,
            inorder,
            inorder_val_to_idx,
        );
        Some(root)
    }
}
