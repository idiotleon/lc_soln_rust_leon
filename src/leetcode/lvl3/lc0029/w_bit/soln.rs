/// @author: Leon
/// https://leetcode.com/problems/divide-two-integers/
/// Time Complexity:    O(1)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/divide-two-integers/discuss/1085076/Rust-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let is_negative = (dividend < 0) != (divisor < 0);
        let (mut dividend, mut divisor) = (i64::from(dividend).abs(), i64::from(divisor).abs());
        let mut ans = 0_i64;
        let mut n = 1;
        while (divisor << 1) <= dividend {
            divisor <<= 1;
            n <<= 1;
        }
        while n > 0 {
            if dividend >= divisor {
                ans += n;
                dividend -= divisor;
            }
            divisor >>= 1;
            n >>= 1;
        }
        (if is_negative {
            -ans
        } else {
            ans.min(i64::from(std::i32::MAX))
        }) as i32
    }
}
