/// @author: Leon
/// https://leetcode.com/problems/pairs-of-songs-with-total-durations-divisible-by-60/
/// Time Complexity:    O(_len_n)
/// Space Complexity:   O(_len_n)
/// Reference:
/// https://leetcode.com/problems/pairs-of-songs-with-total-durations-divisible-by-60/discuss/256738/JavaC%2B%2BPython-Two-Sum-with-K-60
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_pairs_divisible_by60(times: Vec<i32>) -> i32 {
        let _len_t: usize = times.len();
        const VALUE_RANGE: i32 = 500 + 100;
        const DATA_RANGE: usize = 60;
        let mut freqs: Vec<i32> = vec![0; DATA_RANGE];
        let mut cnt: i32 = 0;
        for time in times.into_iter() {
            cnt += freqs[(VALUE_RANGE - time) as usize % DATA_RANGE];
            freqs[time as usize % DATA_RANGE] += 1;
        }
        cnt
    }
}
