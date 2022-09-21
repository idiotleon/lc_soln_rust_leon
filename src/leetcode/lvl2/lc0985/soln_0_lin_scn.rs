/// @author: Leon
/// https://leetcode.com/problems/sum-of-even-numbers-after-queries/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sum_even_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let len_ns: usize = nums.len();
        let mut sum: i32 = nums.iter().filter(|&n| n % 2 == 0).sum::<i32>();
        let mut ans: Vec<i32> = Vec::with_capacity(len_ns);
        for query in queries {
            let val: i32 = query[0];
            let idx_q: usize = query[1] as usize;
            let val_prev: i32 = nums[idx_q];
            let val_cur: i32 = val_prev + val;
            if val_prev.abs() % 2 == 1 {
                if val_cur.abs() % 2 == 0 {
                    sum += val_cur;
                }
            } else {
                if val_cur.abs() % 2 == 1 {
                    sum -= val_prev;
                } else {
                    sum += val;
                }
            }
            nums[idx_q] = val_cur;
            ans.push(sum);
        }
        return ans;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let nums: Vec<i32> = vec![1, 2, 3, 4];
        let queries: Vec<Vec<i32>> = vec![vec![1, 0], vec![-3, 1], vec![-4, 0], vec![2, 3]];
        let expected: Vec<i32> = vec![8, 6, 2, 4];
        let actual: Vec<i32> = Solution::sum_even_after_queries(nums, queries);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_with_sample_input_2_should_return_expected() {
        let nums: Vec<i32> = vec![1];
        let queries: Vec<Vec<i32>> = vec![vec![4, 0]];
        let expected: Vec<i32> = vec![0];
        let actual: Vec<i32> = Solution::sum_even_after_queries(nums, queries);
        assert_eq!(expected, actual);
    }
}
