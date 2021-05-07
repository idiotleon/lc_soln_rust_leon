/// https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/
///
/// Time Complexity:     O()
/// Space Complexity:    O()
///
/// Reference:
///  https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/discuss/1194315/Rust-recursive-solution
use crate::leetcode::util::data_structure::{list_node::ListNode, tree_node::TreeNode};
use std::rc::Rc;
use std::cell::RefCell;

#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut len = 0;
        let mut cur = &head;
        while let Some(n) = cur{
            len += 1;
            cur = &n.next;
        }
        
        let mut head = head;
        Self::dfs(&mut head, len)
    }
    
    fn dfs(list: &mut Option<Box<ListNode>>, len: usize) -> Option<Rc<RefCell<TreeNode>>>{
        if len == 0{
            return None;
        }
        
        let left = Self::dfs(list, len / 2);
        if let Some(head) = list{
            let mut node = TreeNode::new(head.val);
            *list = head.next.take();
            node.left = left;
            node.right = Self::dfs(list, len - len / 2 - 1);
            Some(Rc::new(RefCell::new(node)))
        }else{
            None
        }
    }
}