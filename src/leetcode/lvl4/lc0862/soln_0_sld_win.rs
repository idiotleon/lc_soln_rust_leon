use std::collections::VecDeque;
/// @author: Leon
/// https://leetcode.com/problems/shortest-subarray-with-sum-at-least-k/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(`len_n`)
/// Reference:
/// https://leetcode.com/problems/shortest-subarray-with-sum-at-least-k/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let len_n: usize = nums.len();
        let k: i64 = k as i64;
        // the upper bound of the length of `nums`
        const RANGE: usize = 1e5 as usize + 7;
        let mut shortest: usize = RANGE;
        let prefix_sums: Vec<i64> = {
            let mut prefix_sums: Vec<i64> = vec![0; len_n + 1];
            for (idx, num) in nums.into_iter().enumerate() {
                prefix_sums[1 + idx] = prefix_sums[idx] + num as i64;
            }
            prefix_sums
        };
        let mut deque: VecDeque<usize> = VecDeque::new();
        for idx in 0..len_n + 1 {
            while !deque.is_empty() && prefix_sums[idx] - prefix_sums[*deque.front().unwrap()] >= k
            {
                shortest = std::cmp::min(shortest, idx - deque.pop_front().unwrap());
            }
            while !deque.is_empty() && prefix_sums[idx] <= prefix_sums[*deque.back().unwrap()] {
                deque.pop_back();
            }
            deque.push_back(idx);
        }
        if shortest == RANGE {
            -1
        } else {
            shortest as i32
        }
    }
}
