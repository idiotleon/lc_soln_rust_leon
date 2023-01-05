/// @author: Leon
/// https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/
/// Time Complexity:    O(`_len_pts` * lg(`_len_pts`))
/// Space Complexity:   O(1)
/// Reference:
/// http://zxi.mytechroad.com/blog/geometry/leetcode-452-minimum-number-of-arrows-to-burst-balloons/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let _len_pts: usize = points.len();
        let sorted: Vec<Vec<i32>> = {
            let mut sorted = points;
            sorted.sort_by_key(|pt| pt[1]);
            sorted
        };
        let mut end_prev: i32 = sorted[0][1];
        let mut cnt: i32 = 1;
        for pt in sorted {
            let start_cur = pt[0];
            let end_cur = pt[1];
            if start_cur > end_prev {
                end_prev = end_cur;
                cnt += 1;
            }
        }
        return cnt;
    }
}
