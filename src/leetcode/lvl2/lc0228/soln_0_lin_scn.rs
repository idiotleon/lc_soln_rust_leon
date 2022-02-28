/// @author: Leon
/// https://leetcode.com/problems/summary-ranges/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(`len_n`) / O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let len_n: usize = nums.len();
        let mut ans: Vec<String> = Vec::new();
        let mut idx: usize = 0;
        while idx < len_n {
            let start = idx;
            while idx + 1 < len_n && nums[idx + 1] == 1 + nums[idx] {
                idx += 1;
            }
            if nums[idx] > nums[start] {
                let mut tmp: String = nums[start].to_string();
                tmp.push_str(&"->");
                tmp.push_str(&nums[idx].to_string());
                ans.push(tmp);
            } else if nums[idx] == nums[start] {
                ans.push(nums[idx].to_string());
            }
            idx += 1;
        }
        ans
    }
}
