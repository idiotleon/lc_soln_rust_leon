/// https://leetcode.com/problems/maximal-rectangle/
/// Time Complexity:    O(`_len_r` * `len_c`)
/// Space Complexity:   O(`len_c`)
/// Reference:
/// https://leetcode.com/problems/maximal-rectangle/discuss/29064/A-O(n2)-solution-based-on-Largest-Rectangle-in-Histogram
use std::collections::VecDeque;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    const ONE: char = '1';
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }
        let _len_r = matrix.len();
        let len_c = matrix[0].len();
        let mut max: i32 = 0;
        let mut heights: Vec<i32> = vec![0; len_c];
        for row in matrix {
            for (idx_c, val) in row.into_iter().enumerate() {
                if val == Self::ONE {
                    heights[idx_c] += 1;
                } else {
                    heights[idx_c] = 0;
                }
            }
            max = std::cmp::max(max, Self::get_max_area(&heights));
        }
        max
    }
    fn get_max_area(heights: &Vec<i32>) -> i32 {
        let len_h = heights.len();
        let mut stk: VecDeque<usize> = VecDeque::new();
        let mut max: i32 = 0;
        let mut idx: usize = 0;
        while idx < len_h {
            while !stk.is_empty() && heights[idx] < heights[*stk.back().unwrap()] {
                let shortest = heights[stk.pop_back().unwrap()];
                let width = (idx
                    - if stk.is_empty() {
                        0
                    } else {
                        stk.back().unwrap() + 1
                    }) as i32;
                let area = shortest * width;
                max = std::cmp::max(max, area);
            }
            stk.push_back(idx);
            idx += 1;
        }
        while !stk.is_empty() {
            let shortest = heights[stk.pop_back().unwrap()];
            let width = (len_h
                - if stk.is_empty() {
                    0
                } else {
                    stk.back().unwrap() + 1
                }) as i32;
            let area = shortest * width;
            max = std::cmp::max(max, area);
        }
        max
    }
}
