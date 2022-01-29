use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/number-of-boomerangs/
/// Time Complexity:    O(`_len_pts` ^ 2)
/// Space Complexity:   O(`_len_pts`)
/// Reference:
/// https://leetcode.com/problems/number-of-boomerangs/discuss/92861/Clean-java-solution%3A-O(n2)-166ms
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let _len_pts: usize = points.len();
        let mut cnt: i32 = 0;
        let mut dist_to_freq: HashMap<i32, i32> = HashMap::new();
        for (idx1, pt1) in points.iter().enumerate() {
            for (idx2, pt2) in points.iter().enumerate() {
                if idx1 == idx2 {
                    continue;
                }
                let dist = Self::get_distance(pt1, pt2);
                *dist_to_freq.entry(dist).or_default() += 1;
            }
            for (_, freq) in dist_to_freq.iter() {
                cnt += freq * (freq - 1);
            }
            dist_to_freq.clear();
        }
        cnt
    }
    fn get_distance(pt1: &Vec<i32>, pt2: &Vec<i32>) -> i32 {
        let delta_x = pt1[0] - pt2[0];
        let delta_y = pt1[1] - pt2[1];
        delta_x * delta_x + delta_y * delta_y
    }
}
