/// @author: Leon
/// https://leetcode.com/problems/check-if-an-array-is-consecutive/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(1)
/// Note:
/// The sum of arithmetic sequence cannot conclude that all elements are distinct
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_consecutive(nums: Vec<i32>) -> bool {
        let len_ns: i32 = nums.len() as i32;
        let (max, min, sum): (i32, i32, i32) = {
            let mut max: i32 = nums[0];
            let mut min: i32 = nums[0];
            let mut sum: i32 = 0;
            for num in nums {
                max = std::cmp::max(max, num);
                min = std::cmp::min(min, num);
                sum += num;
            }
            (max, min, sum)
        };
        max - min + 1 == len_ns && len_ns * (max + min) / 2 == sum
    }
}
