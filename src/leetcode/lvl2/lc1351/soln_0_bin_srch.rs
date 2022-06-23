/// @author: Leon
/// https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix/
/// Time Complexity:    O(`len_r` * lg(`_len_c`)
/// Space Complexity：  O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let _len_r = grid.len();
        let len_c = grid[0].len();
        let mut cnt: usize = 0;
        for row in grid {
            let idx = Self::bs_lower_bound(row);
            let len = if idx >= len_c { 0 } else { len_c - idx };
            cnt += len;
        }
        cnt as i32
    }
    fn bs_lower_bound(nums: Vec<i32>) -> usize {
        let len_n: usize = nums.len();
        let mut lo: usize = 0;
        let mut hi: usize = len_n;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if nums[mid] < 0 {
                hi = mid;
            } else if nums[mid] >= 0 {
                lo = mid + 1;
            }
        }
        lo
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let grid = vec![
            vec![4, 3, 2, -1],
            vec![3, 2, 1, -1],
            vec![1, 1, -1, -2],
            vec![-1, -1, -2, -3],
        ];
        let actual = Solution::count_negatives(grid);
        let expected = 8;
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let grid = vec![vec![3, 2], vec![1, 0]];
        let actual = Solution::count_negatives(grid);
        let expected = 0;
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_test_case_10() {
        let grid = vec![vec![5, 1, 0], vec![-5, -5, -5]];
        let actual = Solution::count_negatives(grid);
        let expected = 3;
        assert_eq!(expected, actual);
    }
}
