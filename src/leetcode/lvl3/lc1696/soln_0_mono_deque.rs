use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/jump-game-vi/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(`len_ns`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_result(mut nums: Vec<i32>, k: i32) -> i32 {
        let len_ns: usize = nums.len();
        let k: usize = k as usize;
        let mut score: i32 = 0;
        let mut deque: VecDeque<usize> = VecDeque::with_capacity(len_ns);
        for idx in (0..len_ns).rev() {
            score = nums[idx]
                + if let Some(&first) = deque.front() {
                    nums[first]
                } else {
                    0
                };
            while !deque.is_empty() && score > nums[*deque.back().unwrap()] {
                deque.pop_back();
            }
            deque.push_back(idx);
            if *deque.front().unwrap() >= idx + k {
                deque.pop_front();
            }
            nums[idx] = score;
        }
        score
    }
}
