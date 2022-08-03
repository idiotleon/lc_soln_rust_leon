/// @author: Leon
/// https://leetcode.com/problems/make-array-zero-by-subtracting-equal-amounts/
/// Time Complexity:    O(`RANGE`) ~ O(1)
/// Space Complexity:   O(`RANGE`) ~ O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let _len_ns: usize = nums.len();
        const RANGE: usize = 100 + 1;
        let freqs: Vec<u8> = {
            let mut freqs = vec![0; RANGE];
            for num in nums {
                freqs[num as usize] += 1;
            }
            freqs
        };
        let mut ans: u8 = 0;
        for idx in 1..RANGE {
            if freqs[idx] > 0 {
                ans += 1;
            }
        }
        ans as i32
    }
}
