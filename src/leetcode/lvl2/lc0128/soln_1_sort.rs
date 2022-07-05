/// @author: Leon
/// https://leetcode.com/problems/longest-consecutive-sequence/
/// Time Complexity:    O(`len_ns` * lg(`len_ns`))
/// Space Complexity:   O(`len_ns`)
/// Note:
/// according to the requirements of the problem,
/// only O(N) is allowed
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let len_ns: usize = nums.len();
        let nums: Vec<i32> = {
            let mut sorted = nums;
            sorted.sort();
            sorted
        };
        let mut cur = nums[0];
        let mut len: i32 = 1;
        let mut longest: i32 = 1;
        for idx in 1..len_ns {
            if nums[idx - 1] == nums[idx] {
                continue;
            }
            if nums[idx] == cur + 1 {
                cur += 1;
                len += 1;
                longest = std::cmp::max(longest, len);
            } else {
                cur = nums[idx];
                len = 1;
            }
        }
        longest
    }
}
