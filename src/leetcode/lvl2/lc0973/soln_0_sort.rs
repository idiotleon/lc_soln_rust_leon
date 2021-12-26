/// https://leetcode.com/problems/k-closest-points-to-origin/
/// Time Complexity:    O(`_len_pts` * lg(`_len_pts`))
/// Space Complexity:   O(1) / O(`_len_pts`)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let _len_pts: usize = points.len();
        let mut pts = points;
        pts.sort_by_key(|k| k[0].pow(2) + k[1].pow(2));
        pts[..k as usize].to_vec()
    }
}
