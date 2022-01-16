/// @author: Leon
/// https://leetcode.com/problems/maximize-distance-to-closest-person/
/// Time Complexity:    O(`len_sts`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/maximize-distance-to-closest-person/discuss/137912/JavaC%2B%2BPython-One-pass-Easy-Understood
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let len_sts: usize = seats.len();
        const SEATED: i32 = 1;
        const _EMPTY: i32 = 0;
        let mut last: isize = -1;
        let mut longest: usize = 0;
        for (idx, seat) in seats.into_iter().enumerate() {
            if seat == SEATED {
                longest = if last < 0 {
                    idx
                } else {
                    std::cmp::max(longest, (idx - last as usize) / 2)
                };
                last = idx as isize;
            }
        }
        longest = std::cmp::max(longest, len_sts - last as usize - 1);
        longest as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let seats: Vec<i32> = vec![1, 0, 0, 0, 1, 0, 1];
        let actual: i32 = Solution::max_dist_to_closest(seats);
        let expected: i32 = 2;
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let seats: Vec<i32> = vec![1, 0, 0, 0];
        let actual: i32 = Solution::max_dist_to_closest(seats);
        let expected: i32 = 3;
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_3() {
        let seats: Vec<i32> = vec![0, 1];
        let actual: i32 = Solution::max_dist_to_closest(seats);
        let expected: i32 = 1;
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_test_case_33() {
        let seats: Vec<i32> = vec![1, 0, 1];
        let actual: i32 = Solution::max_dist_to_closest(seats);
        let expected: i32 = 1;
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_edge_case_1() {
        let seats: Vec<i32> = vec![0, 1];
        let actual: i32 = Solution::max_dist_to_closest(seats);
        let expected: i32 = 1;
        assert_eq!(expected, actual);
    }
    // according to the description of the problem:
    // the size of the array must be equal and larger than 2,
    // and there is at least one seat occupied, and at least one empty seat
}
