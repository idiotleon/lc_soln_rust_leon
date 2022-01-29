use std::collections::VecDeque;
/// @author: Leon
/// https://leetcode.com/problems/largest-rectangle-in-histogram/
/// Time Complexity:    O(`len_hts`)
/// Space Complexity:   O(`len_hts`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let len_hts: usize = heights.len();
        let mut stk: VecDeque<usize> = VecDeque::new();
        let mut largest: i32 = 0;
        let mut idx: usize = 0;
        while idx < len_hts {
            while !stk.is_empty() && heights[idx] < heights[*stk.back().unwrap()] {
                let shortest = heights[stk.pop_back().unwrap()];
                let width = (idx
                    - if stk.is_empty() {
                        0
                    } else {
                        stk.back().unwrap() + 1
                    }) as i32;
                let area = shortest * width;
                largest = std::cmp::max(largest, area);
            }
            stk.push_back(idx);
            idx += 1;
        }
        while !stk.is_empty() {
            let shortest = heights[stk.pop_back().unwrap()];
            let width = (len_hts
                - if stk.is_empty() {
                    0
                } else {
                    stk.back().unwrap() + 1
                }) as i32;
            let area = shortest * width;
            largest = std::cmp::max(largest, area);
        }
        largest
    }
}
