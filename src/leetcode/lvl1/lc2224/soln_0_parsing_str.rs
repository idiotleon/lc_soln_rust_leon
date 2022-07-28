/// @author: Leon
/// https://leetcode.com/problems/minimum-number-of-operations-to-convert-time/
/// Time Complexity:    O(1)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/minimum-number-of-operations-to-convert-time/discuss/1908779/Convert-to-minutess
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn convert_time(current: String, correct: String) -> i32 {
        let diff = Self::convert_to_min(correct) - Self::convert_to_min(current);
        diff / 60 + diff % 60 / 15 + diff % 15 / 5 + diff % 5
    }
    fn convert_to_min(time: String) -> i32 {
        let digits: Vec<i32> = time
            .chars()
            .filter(|&ch| ch != ':')
            .map(|ch| ch as i32 - '0' as i32)
            .collect::<Vec<i32>>();
        digits[0] * 600 + digits[1] * 60 + digits[2] * 10 + digits[3]
    }
}
