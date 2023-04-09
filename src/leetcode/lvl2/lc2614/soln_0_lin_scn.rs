/// @author: Leon
/// https://leetcode.com/problems/prime-in-diagonal/
/// Time Complexity:    O(`len_rs`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
        let len_rs: usize = nums.len();
        let len_cs: usize = nums[0].len();
        const RANGE: i32 = 4 * 1e6 as i32 + 7;
        let mut ans: i32 = -RANGE;
        for r in 0..len_rs {
            let num1 = nums[r][r];
            if Self::is_prime(num1) {
                ans = std::cmp::max(ans, num1);
            }
            let num2 = nums[r][len_cs - r - 1];
            if Self::is_prime(num2) {
                ans = std::cmp::max(ans, num2);
            }
        }
        return if ans == -RANGE { 0 } else { ans };
    }
    fn is_prime(num: i32) -> bool {
        if num <= 1 {
            return false;
        }
        let mut n = 2;
        while n * n <= num {
            if num % n == 0 {
                return false;
            }
            n += 1;
        }
        return true;
    }
}
