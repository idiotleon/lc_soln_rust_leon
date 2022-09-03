/// @author: Leon
/// https://leetcode.com/problems/find-nearest-point-that-has-the-same-x-or-y-coordinate/
/// Time Complexity:    O(`_len_pts`)
/// Space Compleixty:   O(0)
/// Reference:
/// https://leetcode.com/problems/find-nearest-point-that-has-the-same-x-or-y-coordinate/discuss/1096346/JavaPython-3-Straight-forward-codes.
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        let _len_pts: usize = points.len();
        const RANGE: i32 = 1e4 as i32 + 7;
        let mut ans: i32 = -1;
        let mut nearest: i32 = RANGE;
        for (idx, pt) in points.into_iter().enumerate() {
            let dx = x - pt[0];
            let dy = y - pt[1];
            let dist = (dx + dy).abs();
            if dx * dy == 0 && dist < nearest {
                nearest = dist;
                ans = idx as i32;
            }
        }
        return ans;
    }
}
