/// @author: Leon
/// https://leetcode.com/problems/single-number-iii/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let len_n = nums.len();
        let res = {
            let mut res = nums[0];
            for idx in 1..len_n {
                res ^= nums[idx];
            }
            res
        };
        let last_xor_digit = res & -res;
        let ans: i32 = {
            let mut ans = res;
            for num in nums {
                if num & last_xor_digit > 0 {
                    ans ^= num;
                }
            }
            ans
        };
        vec![ans, res ^ ans]
    }
}
