/// @author: Leon
/// https://leetcode.com/problems/find-greatest-common-divisor-of-array/
/// Time Complexity:    O(`_len_ns`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let _len_ns: usize = nums.len();
        let (min, max) = {
            let mut min = nums[0];
            let mut max = nums[1];
            for num in nums {
                min = std::cmp::min(min, num);
                max = std::cmp::max(max, num);
            }
            (min, max)
        };
        return Self::gcd(min, max);
    }
    fn gcd(num1: i32, num2: i32) -> i32 {
        if num2 == 0 {
            return num1;
        }
        return Self::gcd(num2, num1 % num2);
    }
}
