/// https://leetcode.com/problems/add-two-numbers/
///
/// Time Complexity:    O(len1 + len2) ~ O(max(len1, len2))
/// Space ComplexitY;   O(len1 + len2) ~ O(max(len1, len2)))
///
/// Reference:
/// https://leetcode.com/problems/add-two-numbers/discuss/469977/Simple-Rust-solution-less0ms-2.1MBgreater
use crate::leetcode::util::data_structure::list_node::ListNode;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (Some(n), None) | (None, Some(n)) => Some(n),
            (Some(n1), Some(n2)) => {
                let sum = n1.val + n2.val;
                let (rem, carry) = if sum < 10 {
                    (sum, None)
                } else {
                    (sum - 10, Some(Box::new(ListNode::new(1))))
                };

                Some(Box::new(ListNode {
                    val: rem,
                    next: Self::add_two_numbers(Self::add_two_numbers(n1.next, carry), n2.next),
                }))
            }
        }
    }
}
