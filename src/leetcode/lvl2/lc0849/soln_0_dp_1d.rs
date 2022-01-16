/// @author: Leon
/// https://leetcode.com/problems/maximize-distance-to-closest-person/
/// Time Complexity:    O(`len_sts`)
/// Space Complexity:   O(`len_sts`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let len_sts: usize = seats.len();
        const RANGE: i32 = 2e4 as i32 + 7;
        const SEATED: i32 = 1;
        const EMPTY: i32 = 0;
        let lo_lens: Vec<usize> = {
            let mut res: Vec<usize> = vec![0; len_sts];
            let mut len: usize = if seats[len_sts - 1] == EMPTY {
                len_sts
            } else {
                0
            };
            for (idx, &num) in seats.iter().enumerate().rev() {
                if num == SEATED {
                    len = 0;
                } else {
                    len += 1;
                }
                res[idx] = len;
            }
            res
        };
        let hi_lens: Vec<usize> = {
            let mut res: Vec<usize> = vec![0; len_sts];
            let mut len: usize = if seats[0] == EMPTY { len_sts } else { 0 };
            for (idx, &num) in seats.iter().enumerate() {
                if num == SEATED {
                    len = 0;
                } else {
                    len += 1;
                }
                res[idx] = len;
            }
            res
        };
        let ans: i32 = {
            let mut res: i32 = 0;
            for idx in 0..len_sts {
                let min_len = std::cmp::min(lo_lens[idx], hi_lens[idx]);
                res = std::cmp::max(res, min_len as i32);
            }
            res
        };
        ans
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
}
