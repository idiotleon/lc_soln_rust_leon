/// @author: Leon
/// https://leetcode.com/problems/sequential-digits/
/// Time Complexity:    O(`_len_cs`)
/// Space Complexity:   O(`_len_cs`) ~ O(1)
/// Reference:
/// https://leetcode.com/problems/sequential-digits/discuss/451851/Java-Just-a-joke
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        const CANDIDATES: &[i32] = &[
            12, 23, 34, 45, 56, 67, 78, 89, 123, 234, 345, 456, 567, 678, 789, 1234, 2345, 3456,
            4567, 5678, 6789, 12345, 23456, 34567, 45678, 56789, 123456, 234567, 345678, 456789,
            1234567, 2345678, 3456789, 12345678, 23456789, 123456789,
        ];
        let _len_cs: usize = CANDIDATES.len();
        let mut ans: Vec<i32> = Vec::new();
        for &num in CANDIDATES {
            if low <= num && num <= high {
                ans.push(num);
            }
        }
        ans
    }
}
