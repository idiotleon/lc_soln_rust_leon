/// @author: Leon
/// https://leetcode.com/problems/sum-of-digits-in-base-k/
/// 
/// Time Complexity:    O(`n` / `k`)
/// Space Complexity:   O(1)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sum_base(n: i32, k: i32) -> i32 {
        let mut n = n;
        let mut sum = 0;
        
        while n > 0{
            let digit = n % k;
            n /= k;
            sum += digit;
        }
        
        sum
    }
}