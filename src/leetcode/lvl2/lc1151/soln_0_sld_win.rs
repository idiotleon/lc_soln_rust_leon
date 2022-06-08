/// @author: Leon
/// https://leetcode.com/problems/minimum-swaps-to-group-all-1s-together/
/// Time Complexity:    O(`len_d`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_swaps(data: Vec<i32>) -> i32 {
        let len_d: usize = data.len();
        let sum_ones: i32 = data.iter().sum();
        if sum_ones == 0 {
            return 0;
        }
        let mut cnt_ones: i32 = 0;
        // represented by `hi - sum_ones as usize`
        let mut _lo: usize = 0;
        let mut hi: usize = 0;
        let mut most: i32 = 0;
        while hi < len_d {
            cnt_ones += data[hi];
            if hi > sum_ones as usize - 1 {
                cnt_ones -= data[hi - sum_ones as usize];
            }
            most = std::cmp::max(most, cnt_ones);
            hi += 1;
        }
        sum_ones - most
    }
}
