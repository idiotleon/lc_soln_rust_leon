/// @author: Leon
///
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximums_spliced_array(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        std::cmp::max(
            Self::get_max_sum(&nums1, &nums2),
            Self::get_max_sum(&nums2, &nums1),
        )
    }
    fn get_max_sum(nums1: &Vec<i32>, nums2: &Vec<i32>) -> i32 {
        let len_n: usize = nums1.len();
        let sum_total: i32 = nums1.iter().sum::<i32>();
        let mut sum = sum_total;
        // the running and accumulated extra
        let mut extra: i32 = 0;
        let mut largest: i32 = sum_total;
        for idx in 0..len_n {
            let diff: i32 = nums2[idx] - nums1[idx];
            extra += diff;
            if extra < 0 {
                // to reset everything
                extra = 0;
                sum = sum_total;
            } else {
                sum += diff;
                largest = std::cmp::max(largest, sum);
            }
        }
        largest
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let nums1: Vec<i32> = vec![60, 60, 60];
        let nums2: Vec<i32> = vec![10, 90, 10];
        let actual = Solution::maximums_spliced_array(nums1, nums2);
        let expected = 210;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_with_sample_input_2_should_return_expected() {
        let nums1: Vec<i32> = vec![20, 40, 20, 70, 30];
        let nums2: Vec<i32> = vec![50, 20, 50, 40, 20];
        let actual = Solution::maximums_spliced_array(nums1, nums2);
        let expected = 220;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_with_test_case_should_return_expected() {
        let nums1: Vec<i32> = vec![10, 20, 50, 15, 30, 10];
        let nums2: Vec<i32> = vec![40, 20, 10, 100, 10, 10];
        let actual = Solution::maximums_spliced_array(nums1, nums2);
        let expected = 230;
        assert_eq!(expected, actual);
    }
}
