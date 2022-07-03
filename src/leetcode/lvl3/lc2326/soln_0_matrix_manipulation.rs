use crate::leetcode::util::data_structure::linked_list::single::list_node::ListNode;

/// @author: Leon
/// https://leetcode.com/problems/spiral-matrix-iv/
/// Time Complexity:    O(`m` * `n`)
/// Space Complexity:   O(`m` * `n`) / O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let m: usize = m as usize;
        let n: usize = n as usize;
        let mut ans: Vec<Vec<i32>> = vec![vec![0; n]; m];
        let mut top: isize = 0;
        let mut bottom: isize = m as isize - 1;
        let mut left: isize = 0;
        let mut right: isize = n as isize - 1;
        let mut cur = head;
        loop {
            for c in left..=right {
                ans[top as usize][c as usize] = if let Some(n) = cur {
                    let val = n.val;
                    cur = n.next;
                    val
                } else {
                    -1
                };
            }
            top += 1;
            if left > right || top > bottom {
                break;
            }
            for r in top..=bottom {
                ans[r as usize][right as usize] = if let Some(n) = cur {
                    let val = n.val;
                    cur = n.next;
                    val
                } else {
                    -1
                };
            }
            right -= 1;
            if left > right || top > bottom {
                break;
            }
            for c in (left..=right).rev() {
                ans[bottom as usize][c as usize] = if let Some(n) = cur {
                    let val = n.val;
                    cur = n.next;
                    val
                } else {
                    -1
                };
            }
            bottom -= 1;
            if left > right || top > bottom {
                break;
            }
            for r in (top..=bottom).rev() {
                ans[r as usize][left as usize] = if let Some(n) = cur {
                    let val = n.val;
                    cur = n.next;
                    val
                } else {
                    -1
                };
            }
            left += 1;
            if left > right || top > bottom {
                break;
            }
        }
        ans
    }
}
