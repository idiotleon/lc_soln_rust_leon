/// @author: Leon
/// https://leetcode.com/problems/find-the-duplicate-number/
/// Time Complexity:    O(`_len_n`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/find-the-duplicate-number/discuss/1894339/C%2B%2B-8-Different-solutions-to-this-problem-or-Do-you-have-another-one
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let _len_n: usize = nums.len();
        let mut slow: i32 = nums[0];
        let mut fast: i32 = nums[slow as usize];
        while slow != fast {
            slow = nums[slow as usize];
            fast = nums[nums[fast as usize] as usize];
        }
        slow = 0;
        while slow != fast {
            slow = nums[slow as usize];
            fast = nums[fast as usize];
        }
        fast
    }
}
