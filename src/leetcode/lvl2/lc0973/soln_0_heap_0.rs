use std::collections::BinaryHeap;

/// https://leetcode.com/problems/k-closest-points-to-origin/
/// Time Complexity:    O(`_len_pts` * lg(`k`))
/// Space Complexity:   O(`k`)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let _len_pts: usize = points.len();
        let mut heap: BinaryHeap<(i64, Vec<i32>)> = BinaryHeap::new();
        for pt in points {
            let dist = pt[0] as i64 * pt[0] as i64 + pt[1] as i64 * pt[1] as i64;
            heap.push((dist, pt));
            if heap.len() > k as usize {
                heap.pop();
            }
        }
        let ans: Vec<Vec<i32>> = {
            let mut tmp: Vec<Vec<i32>> = Vec::new();
            while let Some((_, pt)) = heap.pop() {
                tmp.push(pt);
            }
            tmp
        };
        ans
    }
}
