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
            let mut sorted: Vec<Vec<i32>> = points;
            sorted.sort_by_key(|pt| pt[1]);
            sorted
        };
        let mut prev_end: i32 = sorted[0][1];
        let mut arrow: i32 = 1;
        for pt in sorted {
            if pt[0] > prev_end {
                prev_end = pt[1];
                arrow += 1;
            }
        }
        arrow
    }
}
