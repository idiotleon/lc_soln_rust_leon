struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let len_ns: usize = nums.len();
        let max: i32 = *nums.iter().max().unwrap();
        let freq_max: i32 = nums.iter().filter(|&&n| n == max).count() as i32;
        if freq_max < k {
            return 0;
        }
        let mut cnt: i64 = 0;
        for freq_exp in k..=freq_max {
            let mut hi: usize = 0;
            let mut cnt_max: i32 = 0;
            while hi < len_ns {
                if nums[hi] == max {
                    cnt_max += 1;
                }
                if cnt_max >= freq_exp {
                    cnt += 1;
                }
                let mut lo: usize = 0;
                while lo <= hi && cnt_max >= freq_exp {
                    cnt += 1;
                    if nums[lo] == max {
                        cnt_max -= 1;
                    }
                    lo += 1;
                }
                hi += 1;
            }
        }
        return cnt;
    }
}

#[allow(dead_code, unused_imports)]
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let nums: Vec<i32> = vec![1, 3, 2, 3, 3];
        let k: i32 = 2;
        let freq_expected: i64 = Solution::count_subarrays(nums, k);
        let actual: i64 = 6;
        assert_eq!(freq_expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {}
    #[test]
    fn it_works_with_sample_input_3() {}
    #[test]
    fn it_works_with_sample_input_4() {}
    #[test]
    fn it_works_with_test_input_1() {
        let nums: Vec<i32> = vec![
            61, 23, 38, 23, 56, 40, 82, 56, 82, 82, 82, 70, 8, 69, 8, 7, 19, 14, 58, 42, 82, 10,
            82, 78, 15, 82,
        ];
        let k: i32 = 2;
        let expected: i64 = 224;
        let actual: i64 = Solution::count_subarrays(nums, k);
        assert_eq!(expected, actual);
    }
}
