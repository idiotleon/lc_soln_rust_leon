use crate::leetcode::util::data_structure::linked_list::single::list_node::ListNode;

/// @author: Leon
/// https://leetcode.com/problems/reverse-linked-list/
/// Time Complexity:    O(L)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/reverse-nodes-in-k-group/discuss/183356/Java-O(n)-solution-with-super-detailed-explanation-and-illustration
/// https://leetcode.com/problems/binary-tree-upside-down/discuss/49406/Java-recursive-(O(logn)-space)-and-iterative-solutions-(O(1)-space)-with-explanation-and-figure/156816
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev: Option<Box<ListNode>> = None;
        let mut cur = head;
        while let Some(mut c) = cur {
            let nxt = c.next;
            c.next = prev;
            prev = Some(c);
            cur = nxt;
        }
        prev
    }
}
