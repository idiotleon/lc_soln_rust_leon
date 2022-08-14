/// @author: Leon
/// https://leetcode.com/problems/two-furthest-houses-with-different-colors/
/// Time Complexity:    O(`_len_cs`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let _len_cs: usize = colors.len();
        let dist1 = Self::get_max_distance(&colors);
        let rev: Vec<i32> = {
            let mut colors = colors;
            colors.reverse();
            colors
        };
        let dist2 = Self::get_max_distance(&rev);
        return std::cmp::max(dist1, dist2) as i32;
    }
    fn get_max_distance(colors: &Vec<i32>) -> usize {
        let len_cs: usize = colors.len();
        let mut lo: usize = 0;
        let hi: usize = len_cs - 1;
        while lo < hi {
            if colors[lo] != colors[hi] {
                return hi - lo;
            }
            lo += 1;
        }
        return 0;
    }
}
