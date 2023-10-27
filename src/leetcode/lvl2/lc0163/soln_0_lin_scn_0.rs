/// @author: Leon
/// https://leetcode.com/problems/missing-ranges/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_missing_ranges(nums: Vec<i32>, lower: i32, upper: i32) -> Vec<Vec<i32>> {
        if nums.is_empty() {
            return vec![vec![lower, upper]];
        }
        if lower == upper {
            return vec![];
        }
        let len_ns: usize = nums.len();
        let mut ans: Vec<Vec<i32>> = Vec::with_capacity(len_ns);
        if lower < nums[0] {
            ans.push(vec![lower, nums[0] - 1]);
        }
        for idx in 0..len_ns - 1 {
            if nums[idx] + 1 == nums[idx + 1] {
                continue;
            }
            ans.push(vec![nums[idx] + 1, nums[idx + 1] - 1]);
        }
        if nums[len_ns - 1] < upper {
            ans.push(vec![nums[len_ns - 1] + 1, upper]);
        }
        return ans;
    }
}
