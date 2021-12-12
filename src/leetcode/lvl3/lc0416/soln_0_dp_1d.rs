/// https://leetcode.com/problems/partition-equal-subset-sum/
/// Time Complexity:    O(`_len_n` * `target`)
/// Space Complexity:   O(`target`)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let _len_n = nums.len();
        let sum: i32 = nums.iter().sum();
        if sum % 2 == 1 {
            return false;
        }
        let target = sum / 2;
        let mut dp = vec![false; 1 + target as usize];
        dp[0] = true;
        for num in nums {
            for sum in (0..=target).rev() {
                if dp[sum as usize] {
                    let sum_new = num + sum;
                    if sum_new > target {
                        continue;
                    }
                    dp[sum_new as usize] = true;
                }
                if dp[target as usize] {
                    return true;
                }
            }
        }
        false
    }
}
