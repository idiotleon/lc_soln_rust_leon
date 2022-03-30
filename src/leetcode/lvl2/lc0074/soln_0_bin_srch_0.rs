/// @author: Leon
/// https://leetcode.com/problems/search-a-2d-matrix/
/// Time Complexity:    O(lg(`len_r` * `len_c`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let len_r: usize = matrix.len();
        let len_c: usize = matrix[0].len();
        let len: usize = len_r * len_c;
        let mut lo: isize = 0;
        let mut hi: isize = len as isize - 1;
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            let r = mid / (len_c as isize);
            let c = mid % (len_c as isize);
            if matrix[r as usize][c as usize] == target {
                return true;
            }
            if matrix[r as usize][c as usize] > target {
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }
        false
    }
}
