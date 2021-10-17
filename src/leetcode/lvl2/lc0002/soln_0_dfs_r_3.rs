/// https://leetcode.com/problems/add-two-numbers/
///
/// Time Complexity:    O(len1 + len2) ~ O(max(len1, len2))
/// Space ComplexitY;   O(len1 + len2) ~ O(max(len1, len2)))
///
/// Reference:
/// https://leetcode.com/problems/add-two-numbers/discuss/469977/Simple-Rust-solution-less0ms-2.1MBgreater
use crate::leetcode::util::data_structure::linked_list::single::list_node::ListNode;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::add_three_numbers(l1, l2, 0)
    }

    fn add_three_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        sum: i32,
    ) -> Option<Box<ListNode>> {
        Some(Box::new(match (l1, l2, sum) {
            (None, None, 0) => None?,
            (None, None, val) => ListNode { next: None, val },
            (a, b, mut sum) => ListNode {
                next: Self::add_three_numbers(
                    a.and_then(|a| {
                        sum += a.val;
                        a.next
                    }),
                    b.and_then(|b| {
                        sum += b.val;
                        b.next
                    }),
                    sum / 10,
                ),
                val: sum % 10,
            },
        }))
    }
}
