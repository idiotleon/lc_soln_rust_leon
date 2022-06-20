use crate::leetcode::util::data_structure::linked_list::single::list_node::ListNode;
/// @author: Leon
/// https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer/
/// Time Compleixty:    O(L)
/// Space Compleixty:   O(1)
/// Reference:
/// https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer/discuss/1615746/Rust-simple
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut h = &head;
        let mut num = 0;
        while let Some(cur) = h {
            num = (num << 1) | cur.val;
            h = &cur.next;
        }
        num
    }
}
