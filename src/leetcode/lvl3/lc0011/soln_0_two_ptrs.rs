/// @author: Leon
/// https://leetcode.com/problems/container-with-most-water/
/// Time Complexity:     O(`len_hs`)
/// Space Complexity:    O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let len_hs = heights.len();
        let mut water: i32 = 0;
        let mut lo: usize = 0;
        let mut hi: usize = len_hs - 1;
        while lo < hi {
            let height = std::cmp::min(heights[lo], heights[hi]) as i32;
            water = std::cmp::max(water, (hi - lo) as i32 * height);
            while lo < hi && heights[lo] <= height {
                lo += 1;
            }
            while lo < hi && heights[hi] <= height {
                hi -= 1;
            }
        }
        water
    }
}
