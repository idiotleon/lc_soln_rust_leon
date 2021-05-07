// @author: Leon 
// https://leetcode.com/problems/container-with-most-water/
//
// Time Complexity:     O(`size`)
// Space Complexity:    O(1)
use std::cmp;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let size = heights.len();
        
        let mut water: i32 = 0;
        let mut lo: usize = 0;
        let mut hi: usize = size - 1;
        
        while lo < hi{
            let height = cmp::min(heights[lo], heights[hi]) as i32;
            water = cmp::max(water, (hi - lo) as i32 * height);
            
            while lo < hi && heights[lo] <= height { lo += 1; }
            while lo < hi && heights[hi] <= height { hi -= 1; }
        }
        
        water
    }
}