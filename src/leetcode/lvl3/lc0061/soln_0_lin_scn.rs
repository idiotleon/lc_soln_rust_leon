use crate::leetcode::util::data_structure::linked_list::single::list_node::ListNode;
/// @author: Leon
/// https://leetcode.com/problems/rotate-list/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/rotate-list/discuss/1008727/Rust-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = match head {
            Some(a) => a,
            None => return None,
        };
        let len = {
            let mut res = 1;
            let mut cur = &head;
            while let Some(ref nxt) = cur.next {
                res += 1;
                cur = nxt;
            }
            res
        };
        let mut target = len - k % len;
        if target == len {
            return Some(head);
        }
        let mut cur = &mut head;
        while target > 1 {
            cur = cur.next.as_mut().unwrap();
            target -= 1;
        }
        let mut res = std::mem::replace(&mut cur.next, None).unwrap();
        let mut cur = &mut res;
        while let Some(ref mut nxt) = cur.next {
            cur = nxt;
        }
        cur.next = Some(head);
        Some(res)
    }
}
