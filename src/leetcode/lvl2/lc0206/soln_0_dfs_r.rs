use crate::leetcode::util::data_structure::linked_list::single::list_node::ListNode;

/// @author: Leon
/// https://leetcode.com/problems/reverse-linked-list/
/// Time Complexity:    O(L)
/// Space Complexity:   O(L)
/// Reference:
/// https://leetcode.com/problems/reverse-linked-list/discuss/954631/Rust-0ms-recursion
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::dfs(head, None)
    }
    fn dfs(head: Option<Box<ListNode>>, reversed: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut node) = head {
            let new_head = std::mem::replace(&mut node.next, reversed);
            Self::dfs(new_head, Some(node))
        } else {
            reversed
        }
    }
}
