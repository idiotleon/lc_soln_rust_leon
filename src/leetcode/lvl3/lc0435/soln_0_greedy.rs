/// @author: Leon
/// https://leetcode.com/problems/non-overlapping-intervals/
/// Time Complexity:    O(`_len_is` * lg(`_len_is`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let _len_is: usize = intervals.len();
        let sorted: Vec<Vec<i32>> = {
            let mut to_sort = intervals;
            to_sort.sort_by_key(|interval| interval[1]);
            to_sort
        };
        let mut cur_end: i32 = i32::MIN;
        let mut cnt: i32 = 0;
        for interval in sorted {
            let start = interval[0];
            let end = interval[1];
            if start >= cur_end {
                cur_end = end;
            } else {
                cnt += 1;
            }
        }
        return cnt;
    }
}
