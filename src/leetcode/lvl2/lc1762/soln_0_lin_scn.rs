/// @author: Leon
/// https://leetcode.com/problems/buildings-with-an-ocean-view/
/// Time Complexity:    O(`len_h`)
/// Space Complexity:   O(1) / O(`len_h`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_buildings(heights: Vec<i32>) -> Vec<i32> {
        let len_h: usize = heights.len();
        let ans: Vec<i32> = {
            let mut res: Vec<i32> = Vec::new();
            res.push(len_h as i32 - 1);
            let mut tallest = heights[len_h - 1];
            for idx in (0..len_h - 1).rev() {
                if heights[idx] > heights[idx + 1] && heights[idx] > tallest {
                    res.push(idx as i32);
                    tallest = heights[idx];
                }
            }
            res.reverse();
            res
        };
        ans
    }
}
