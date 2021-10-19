use crate::leetcode::util::data_structure::linked_list::single::list_node::ListNode;

/// https://leetcode.com/problems/middle-of-the-linked-list/
/// Time Complexity:    O(L)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/middle-of-the-linked-list/discuss/325793/Rust-2Pointers-and-Output-to-Array-0ms-Solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = &head;
        let mut slow = &head;
        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }
        slow.clone()
    }
}
