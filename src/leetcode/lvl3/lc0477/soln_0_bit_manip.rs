/// @author: Leon
/// https://leetcode.com/problems/total-hamming-distance/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/total-hamming-distance/discuss/96226/Java-O(n)-time-O(1)-Space
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let len_ns: usize = nums.len();
        let mut cnt_total: i32 = 0;
        for i in 0..32{
            let mut cnt_bit: i32 = 0;
            for &num in &nums{
                cnt_bit += (num >> i) & 1;
            }
            cnt_total += cnt_bit * (len_ns as i32 - cnt_bit);
        }
        return cnt_total;
    }
}