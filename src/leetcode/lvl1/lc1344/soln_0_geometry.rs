/// @author: Leon
/// https://leetcode.com/problems/angle-between-hands-of-a-clock/
/// Time Complexity:    O(1)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/angle-between-hands-of-a-clock/editorial/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let angle_one_min: f64 = 6.0;
        let angle_one_hour: f64 = 30.0;
        let angle_mins: f64 = angle_one_min * (minutes as f64);
        let angle_hour: f64 = ((hour % 12) as f64 + (minutes as f64) / 60.0) * angle_one_hour;
        let diff = (angle_mins - angle_hour).abs();
        return diff.min(360.0 - diff);
    }
}
