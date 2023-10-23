use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/132-pattern/
/// Time Compelxity:    O(`len_ns`)
/// Space Complexity:   O(`len_ns`)
/// Reference:
/// https://leetcode.com/problems/132-pattern/discuss/94071/Single-pass-C%2B%2B-O(n)-space-and-time-solution-(8-lines)-with-detailed-explanation.
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let len_ns: usize = nums.len();
        let mut pattern3: i32 = i32::MIN;
        let mut stk: VecDeque<i32> = VecDeque::with_capacity(len_ns);
        for idx in (0..len_ns).rev() {
            if nums[idx] < pattern3 {
                return true;
            } else {
                while let Some(&top) = stk.back() {
                    if nums[idx] > top {
                        pattern3 = top;
                        stk.pop_back();
                    } else {
                        break;
                    }
                }
            }
            stk.push_back(nums[idx]);
        }
        return false;
    }
}
