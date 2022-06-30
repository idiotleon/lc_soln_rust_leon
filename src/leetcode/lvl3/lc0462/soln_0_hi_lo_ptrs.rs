/// @author: Leon
/// https://leetcode.com/problems/minimum-moves-to-equal-array-elements-ii/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1)
/// Note:
/// the key is to take the median instead of the average
/// Reference:
/// https://leetcode.com/problems/minimum-moves-to-equal-array-elements-ii/discuss/94937/Java(just-like-meeting-point-problem)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_moves2(nums: Vec<i32>) -> i32 {
        let len_n: usize = nums.len();
        let nums: Vec<i32> = {
            let mut nums = nums;
            nums.sort();
            nums
        };
        let mut lo: usize = 0;
        let mut hi: usize = len_n - 1;
        let mut cnt: i32 = 0;
        while lo < hi {
            cnt += nums[hi] - nums[lo];
            lo += 1;
            hi -= 1;
        }
        cnt
    }
}
