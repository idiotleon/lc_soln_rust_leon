/// https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix/
/// Time Complexity:    O(`len_r` + `len_c`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix/discuss/510249/JavaPython-3-2-similar-O(m-%2B-n)-codes-w-brief-explanation-and-analysis.
/// Similar Probles:
/// https://leetcode.com/problems/search-a-2d-matrix-ii/
/// https://leetcode.com/problems/leftmost-column-with-at-least-a-one/discuss/590240/Java-O(m-+-n)-Follow-the-top-left-part-of-the-contour-of-the-1's.
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let len_r: usize = grid.len();
        let len_c: usize = grid[0].len();
        let mut r: isize = len_r as isize - 1;
        let mut c: usize = 0;
        let mut cnt: u16 = 0;
        while r >= 0 && c < len_c {
            if grid[r as usize][c] < 0 {
                r -= 1;
                cnt += len_c as u16 - c as u16;
            } else {
                c += 1;
            }
        }
        cnt as i32
    }
}
