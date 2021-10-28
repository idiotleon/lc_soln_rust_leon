/// https://leetcode.com/problems/non-negative-integers-without-consecutive-ones/
///
/// Time Complexity:    O(RANGE_I32) ~ O(1)
/// Space Complexity:   O(RANGE_I32) ~ O(1)
///
/// Reference:
/// https://leetcode.com/problems/non-negative-integers-without-consecutive-ones/discuss/103754/C%2B%2B-Non-DP-O(32)-Fibonacci-solution
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_integers(num: i32) -> i32 {
        const RANGE_I32: usize = 32;
        let freqs: Vec<u32> = {
            let mut tmp = vec![0; RANGE_I32];
            tmp[0] = 1;
            tmp[1] = 2;
            for idx in 2..RANGE_I32 {
                tmp[idx] = tmp[idx - 1] + tmp[idx - 2];
            }
            tmp
        };
        let mut cnt: u32 = 0;
        let mut prev_bit: u8 = 0;
        let mut k: i32 = 30;
        while k >= 0 {
            if (num & (1 << k)) > 0 {
                cnt += freqs[k as usize];
                if prev_bit > 0 {
                    return cnt as i32;
                }
                prev_bit = 1;
            } else {
                prev_bit = 0;
            }
            k -= 1;
        }
        1 + cnt as i32
    }
}
