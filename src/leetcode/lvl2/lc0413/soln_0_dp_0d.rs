/// @author: Leon
/// https://leetcode.com/problems/arithmetic-slices/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/arithmetic-slices/discuss/90058/Simple-Java-solution-9-lines-2ms
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let len_n: usize = nums.len();
        let mut cnt: i32 = 0;
        let mut cur: i32 = 0;
        for idx in 2..len_n {
            if nums[idx] - nums[idx - 1] == nums[idx - 1] - nums[idx - 2] {
                cur += 1;
                cnt += cur;
            } else {
                cur = 0;
            }
        }
        cnt
    }
}
