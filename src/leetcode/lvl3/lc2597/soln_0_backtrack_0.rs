use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/the-number-of-beautiful-subsets/
/// Time Complexity:    O(2 ^ `len_ns`)
/// Space Complexity:   O(`len_ns`)
/// Reference:
/// https://leetcode.com/problems/the-number-of-beautiful-subsets/solutions/3363862/c-java-python-evolve-brute-force-to-dp-explained-7-approaches/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        const RANGE: usize = 1000;
        let len_ns: usize = nums.len();
        let sorted: Vec<i32> = {
            let mut nums = nums;
            nums.sort();
            nums
        };
        let mut num_to_freq: HashMap<i32, u16> = HashMap::with_capacity(len_ns);
        return Self::backtrack(0, &sorted, k, &mut num_to_freq) - 1;
    }
    fn backtrack(idx: usize, nums: &Vec<i32>, k: i32, num_to_freq: &mut HashMap<i32, u16>) -> i32 {
        let len_ns: usize = nums.len();
        if idx == len_ns {
            return 1;
        }
        // `nums[idx]` not taken
        let mut res = Self::backtrack(idx + 1, nums, k, num_to_freq);
        // `nums[idx]` taken
        if *num_to_freq.entry(nums[idx] - k).or_default() == 0 {
            *num_to_freq.entry(nums[idx]).or_default() += 1;
            res += Self::backtrack(idx + 1, nums, k, num_to_freq);
            *num_to_freq.entry(nums[idx]).or_default() -= 1;
        }
        return res;
    }
}
