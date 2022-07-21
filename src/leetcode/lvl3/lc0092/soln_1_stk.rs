use crate::leetcode::util::data_structure::linked_list::single::list_node::ListNode;

/// @author: Leon
/// https://leetcode.com/problems/reverse-linked-list-ii/
/// Time Complexity:    O(L)
/// Space Complexity:   O(L)
/// Reference:
/// https://leetcode.com/problems/reverse-linked-list-ii/discuss/1293170/Rust-or-a-poor-man's-Vec-'n'-recreate-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        if left == right {
            return head;
        }
        let mut stk: Vec<i32> = vec![];
        let mut cur = head;
        while let Some(c) = cur {
            stk.push(c.val);
            cur = c.next;
        }
        stk[left as usize - 1..=right as usize - 1].reverse();
        let mut cur: Option<Box<ListNode>> = None;
        while let Some(val) = stk.pop() {
            cur = Some(Box::new(ListNode { next: cur, val }));
        }
        cur
    }
}
