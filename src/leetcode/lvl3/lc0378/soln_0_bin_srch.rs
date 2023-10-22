/// @author: Leon
/// https://leetcode.com/problems/kth-smallest-element-in-a-sorted-matrix/
/// Time Complexity:    O(`len_cs` * lg(MAX - MIN))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let len_rs: usize = matrix.len();
        let len_cs: usize = matrix[0].len();
        let mut lo: i32 = matrix[0][0];
        let mut hi: i32 = matrix[len_rs - 1][len_cs - 1];
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let cnt = Self::get_count(&matrix, mid);
            if cnt >= k {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        return lo;
    }
    fn get_count(matrix: &Vec<Vec<i32>>, target: i32) -> i32 {
        let len_rs: isize = matrix.len() as isize;
        let len_cs: isize = matrix[0].len() as isize;
        let mut cnt: i32 = 0;
        let mut r: isize = 0;
        let mut c: isize = len_cs - 1;
        while r < len_rs {
            while c >= 0 && matrix[r as usize][c as usize] > target {
                c -= 1;
            }
            cnt += c as i32 + 1;
            r += 1;
        }
        return cnt;
    }
}
