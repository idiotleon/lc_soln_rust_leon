/// @author: Leon
/// https://leetcode.com/problems/number-of-students-doing-homework-at-a-given-time/
/// Time Complexity:    O(`_len_ts`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        let _len_ts: usize = start_time.len();
        let mut cnt: i32 = 0;
        for (idx, start) in start_time.into_iter().enumerate() {
            let end = end_time[idx];
            if start <= query_time && query_time <= end {
                cnt += 1;
            }
        }
        return cnt;
    }
}
