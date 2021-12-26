use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

/// https://leetcode.com/problems/k-closest-points-to-origin/
/// Time Complexity:    O()
/// Space Complexity:   O()
///
/// not yet correct
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn k_closest(pts: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut heap: BinaryHeap<Reverse<Point>> = BinaryHeap::new();
        for pt in pts {
            heap.push(Reverse(Point::new(pt)));
            if heap.len() > k as usize {
                heap.pop();
            }
        }
        let ans: Vec<Vec<i32>> = {
            let mut tmp: Vec<Vec<i32>> = Vec::new();
            while let Some(top) = heap.pop() {
                tmp.push(vec![top.0.r, top.0.c]);
            }
            tmp.reverse();
            tmp
        };
        ans
    }
}

#[derive(Eq, PartialEq, PartialOrd)]
struct Point {
    r: i32,
    c: i32,
}

impl Point {
    fn new(pt: Vec<i32>) -> Point {
        Point { r: pt[0], c: pt[1] }
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        let distance_self = self.r as i64 * self.r as i64 + self.c as i64 * self.c as i64;
        let distance_other = other.r as i64 * other.r as i64 + other.c as i64 * other.c as i64;
        distance_self.cmp(&distance_other)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_case_14() {
        let pts = vec![vec![1, 3], vec![-2, 2], vec![2, -2]];
        let k = 2;
        let actual = Solution::k_closest(pts, k);
        let expected = vec![vec![-2, 2], vec![2, -2]];
        assert_eq!(actual, expected);
    }
}
