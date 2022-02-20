/// @author: Leon
/// https://leetcode.com/problems/remove-covered-intervals/
/// Time Complexity:    O(`_len_ins` * lg(`_len_ins`))
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/remove-covered-intervals/discuss/451277/JavaC%2B%2BPython-Sort-Solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let _len_ins: usize = intervals.len();
        let mut cnt: i32 = 0;
        let mut lo: i32 = -1;
        let mut hi: i32 = -1;
        let sorted = {
            let mut sorted = intervals;
            sorted.sort_by_key(|e| e[0]);
            sorted
        };
        for v in sorted {
            if v[0] > lo && v[1] > hi {
                lo = v[0];
                cnt += 1;
            }
            hi = std::cmp::max(hi, v[1]);
        }
        cnt
    }
}
