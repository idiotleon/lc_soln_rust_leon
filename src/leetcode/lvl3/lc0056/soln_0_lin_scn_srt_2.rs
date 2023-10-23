/// @author: Leon
/// https://leetcode.com/problems/merge-intervals/
/// Time Complexity:    O(`len_intvl` * lg(`len_intvl`))
/// Space Complexity:   O(`len_intvl`) / O(1)
/// Reference:
/// https://leetcode.com/problems/merge-intervals/discuss/1148566/Rust-onepass
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let len_intvl = intervals.len();
        let sorted: Vec<Vec<i32>> = {
            let mut tmp = intervals;
            tmp.sort_by(|a, b| a[0].cmp(&b[0]));
            tmp
        };
        let mut merged: Vec<Vec<i32>> = Vec::with_capacity(len_intvl);
        for cur_interval in sorted.into_iter() {
            let prev_interval = match merged.last_mut() {
                Some(prev) => prev,
                None => {
                    merged.push(cur_interval);
                    continue;
                }
            };
            if prev_interval[1] >= cur_interval[0] {
                *prev_interval = vec![
                    prev_interval[0],
                    std::cmp::max(cur_interval[1], prev_interval[1]),
                ];
            } else {
                merged.push(cur_interval);
            }
        }
        return merged;
    }
}
