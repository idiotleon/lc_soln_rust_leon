/// @author: Leon
/// https://leetcode.com/problems/determine-if-a-cell-is-reachable-at-a-given-time/
/// Time Complexity:    O(1)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
        if sx == fx && sy == fy {
            return t >= 2;
        }
        let diff_x: i32 = (fx - sx).abs();
        let diff_y: i32 = (fy - sy).abs();
        let min_gap: i32 = std::cmp::max(0, std::cmp::min(diff_x, diff_y));
        let min_step: i32 = (diff_x + diff_y) - min_gap;
        return min_step <= t;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let sx: i32 = 2;
        let sy: i32 = 4;
        let fx: i32 = 7;
        let fy: i32 = 7;
        let t: i32 = 6;
        let expected: bool = true;
        let actual: bool = Solution::is_reachable_at_time(sx, sy, fx, fy, t);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_test_case_736() {
        let sx: i32 = 1;
        let sy: i32 = 1;
        let fx: i32 = 2;
        let fy: i32 = 2;
        let t: i32 = 1;
        let expected: bool = true;
        let actual: bool = Solution::is_reachable_at_time(sx, sy, fx, fy, t);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_test_case_762() {
        let sx: i32 = 1;
        let sy: i32 = 1;
        let fx: i32 = 1;
        let fy: i32 = 2;
        let t: i32 = 0;
        let expected: bool = false;
        let actual: bool = Solution::is_reachable_at_time(sx, sy, fx, fy, t);
        assert_eq!(expected, actual);
    }
}
