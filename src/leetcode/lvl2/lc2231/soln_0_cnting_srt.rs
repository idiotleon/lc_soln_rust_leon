/// @author: Leon
/// https://leetcode.com/problems/largest-number-after-digit-swaps-by-parity/
/// Time Complexity:    O(1)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn largest_integer(num: i32) -> i32 {
        let mut freqs: Vec<u32> = {
            let mut freqs: Vec<u32> = vec![0; 10];
            let mut r_num: i32 = num;
            while r_num > 0 {
                let digit: usize = (r_num % 10) as usize;
                freqs[digit] += 1;
                r_num /= 10;
            }
            freqs
        };
        let mut r_num: i32 = num;
        let mut idx_even: usize = 0;
        let mut idx_odd: usize = 1;
        let mut ans: i32 = 0;
        let mut factor: i32 = 1;
        while r_num > 0 {
            let digit = r_num % 10;
            if digit % 2 == 1 {
                while freqs[idx_odd] == 0 {
                    idx_odd += 2;
                }
                ans += factor * (idx_odd as i32);
                freqs[idx_odd] -= 1;
            } else {
                while freqs[idx_even] == 0 {
                    idx_even += 2;
                }
                ans += factor * (idx_even as i32);
                freqs[idx_even] -= 1;
            }
            r_num /= 10;
            factor *= 10;
        }
        ans
    }
}
