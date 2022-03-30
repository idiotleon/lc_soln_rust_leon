/// @author: Leon
/// https://leetcode.com/problems/find-the-duplicate-number/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/find-the-duplicate-number/discuss/1894339/C%2B%2B-8-Different-solutions-to-this-problem-or-Do-you-have-another-one
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_duplicate(mut nums: Vec<i32>) -> i32 {
        let len_n: usize = nums.len();
        for i in 0..len_n {
            let idx: usize = nums[i].abs() as usize;
            if nums[idx] < 0 {
                return idx as i32;
            }
            nums[idx] = -nums[idx];
        }
        0
    }
}
