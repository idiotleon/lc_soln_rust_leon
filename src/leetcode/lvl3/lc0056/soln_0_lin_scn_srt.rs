/// https://leetcode.com/problems/merge-intervals/
/// Time Complexity:    O(`_len_intvl`)
/// Space Complexity:   O(`_len_intvl`) / O(1)
/// Reference:
/// https://leetcode.com/problems/merge-intervals/discuss/1148566/Rust-onepass
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let _len_intvl = intervals.len();
        let sorted: Vec<Vec<i32>> = {
            let mut tmp = intervals;
            tmp.sort_by(|a, b| a[0].cmp(&b[0]));
            tmp
        };
        let mut merged: Vec<Vec<i32>> = Vec::new();
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
        merged
    }
}
