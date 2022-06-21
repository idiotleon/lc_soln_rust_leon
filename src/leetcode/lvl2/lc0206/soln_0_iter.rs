use crate::leetcode::util::data_structure::linked_list::single::list_node::ListNode;
/// @author: Leon
/// https://leetcode.com/problems/reverse-linked-list/
/// Time Complexity:    O(L)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/reverse-linked-list/discuss/1008312/Rust-0ms-by-iteration
/// https://leetcode.com/problems/reverse-linked-list/discuss/1814267/Rust-or-0ms-or-2.8-or
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ans = None;
        while let Some(mut cur) = head {
            head = std::mem::replace(&mut cur.next, ans);
            ans = Some(cur);
        }
        ans
    }
}
