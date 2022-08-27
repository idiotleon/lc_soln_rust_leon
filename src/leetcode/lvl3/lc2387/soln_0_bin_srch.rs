/// @author: Leon
/// https://leetcode.com/problems/median-of-a-row-wise-sorted-matrix/
/// Time Complexity:    O(`len_rs` * `len_cs`) + O(`len_rs` * lg(`len_cs`))
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/median-of-a-row-wise-sorted-matrix/discuss/2477222/C-Binary-Search
/// https://leetcode.com/problems/median-of-a-row-wise-sorted-matrix/discuss/2475533/Java-Binary-Search-by-value-O(mlogn)-4ms
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn matrix_median(grid: Vec<Vec<i32>>) -> i32 {
        let len_rs: usize = grid.len();
        let len_cs: usize = grid[0].len();
        let range_max: i32 = *grid.iter().map(|v| v.iter().max().unwrap()).max().unwrap();
        let k = len_rs * len_cs / 2;
        let mut lo: i32 = 1;
        let mut hi: i32 = range_max;
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            let mut cnt: usize = 0;
            for r in 0..len_rs {
                let idx = Self::guess(&grid[r], mid);
                cnt += idx;
            }
            if cnt > k {
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }
        return hi;
    }
    fn guess(nums: &Vec<i32>, target: i32) -> usize {
        let len_ns: usize = nums.len();
        let mut lo: usize = 0;
        let mut hi: usize = len_ns;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if nums[mid] < target {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        return lo;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sample_input_1_should_return_expeted() {
        let grid: Vec<Vec<i32>> = vec![vec![1, 1, 2], vec![2, 3, 3], vec![1, 3, 4]];
        let expected: i32 = 2;
        let actual: i32 = Solution::matrix_median(grid);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_sample_input_2_should_return_expeted() {
        let grid: Vec<Vec<i32>> = vec![vec![1, 1, 3, 3, 4]];
        let expected: i32 = 3;
        let actual: i32 = Solution::matrix_median(grid);
        assert_eq!(expected, actual);
    }
}
