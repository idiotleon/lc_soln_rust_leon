use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::util::data_structure::tree::binary::tree_node::TreeNode;

/// @author: Leon
/// https://leetcode.com/problems/leaf-similar-trees/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let vec1: Vec<i32> = {
            let mut vec: Vec<i32> = Vec::new();
            Self::preorder(root1, &mut vec);
            vec
        };
        let vec2: Vec<i32> = {
            let mut vec: Vec<i32> = Vec::new();
            Self::preorder(root2, &mut vec);
            vec
        };
        return vec1 == vec2;
    }
    fn preorder(root: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        if let Some(n) = root {
            if n.borrow().left.is_none() && n.borrow().right.is_none() {
                res.push(n.borrow().val);
            }
            Self::preorder(n.borrow().left.clone(), res);
            Self::preorder(n.borrow().right.clone(), res);
        }
    }
}
