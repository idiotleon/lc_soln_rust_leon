use std::collections::{HashSet, VecDeque};

/// @author: Leon
/// https://leetcode.com/problems/diagonal-traverse-ii/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(`len_ns`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let len_ns: usize = nums.iter().map(|r| r.len()).sum();
        let mut ans: Vec<i32> = vec![0; len_ns];
        let mut queue: VecDeque<(usize, usize)> = {
            let mut queue: VecDeque<(usize, usize)> = VecDeque::with_capacity(len_ns);
            queue.push_back((0, 0));
            queue
        };
        let mut seen: HashSet<(usize, usize)> = {
            let mut seen: HashSet<(usize, usize)> = HashSet::with_capacity(len_ns);
            seen.insert((0, 0));
            seen
        };
        let mut idx: usize = 0;
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            for _ in 0..len_q {
                if let Some((r, c)) = queue.pop_front() {
                    ans[idx] = nums[r][c];
                    idx += 1;
                    if Self::is_valid(r + 1, c, &nums) && seen.insert((r + 1, c)) {
                        queue.push_back((r + 1, c));
                        seen.insert((r + 1, c));
                    }
                    if Self::is_valid(r, c + 1, &nums) && seen.insert((r, c + 1)) {
                        queue.push_back((r, c + 1));
                        seen.insert((r, c + 1));
                    }
                }
            }
        }
        return ans;
    }
    fn is_valid(r: usize, c: usize, nums: &Vec<Vec<i32>>) -> bool {
        let len_rs: usize = nums.len();
        if r >= len_rs {
            return false;
        }
        let len_cs: usize = nums[r].len();
        return c < len_cs;
    }
}
