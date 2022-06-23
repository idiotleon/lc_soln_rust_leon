/// @author: Leon
/// https://leetcode.com/problems/partition-to-k-equal-sum-subsets/
/// Time Complexity:    O(`len_n` * (2 ^ `len_n`)) + O(`len_n` * lg(`len_n`)) ~ O(`len_n` * (2 ^ `len_n`))
/// Space Complxity:    O(2 ^ `len_n`)
/// Reference:
/// https://leetcode.com/problems/partition-to-k-equal-sum-subsets/discuss/335668/DP-with-Bit-Masking-Solution-%3A-Best-for-Interviews
/// https://leetcode.com/problems/partition-to-k-equal-sum-subsets/discuss/108741/solution-with-reference/962804
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let len_n = nums.len();
        let range = 1 << len_n;
        let sum_all: i32 = nums.iter().sum();
        if sum_all % k != 0 {
            return false;
        }
        let mut sums = vec![0; range];
        let mut dp = {
            let mut tmp = vec![false; range];
            tmp[0] = true;
            tmp
        };
        let target = sum_all / k;
        let sorted = {
            let mut nums = nums;
            nums.sort();
            nums
        };
        if *sorted.last().unwrap() > target {
            return false;
        };
        for cur in 0..range {
            if dp[cur] {
                for (idx, &num) in sorted.iter().enumerate() {
                    let next = cur | (1 << idx);
                    if cur == next {
                        continue;
                    }
                    if num <= target - sums[cur] % target {
                        dp[next] = true;
                        sums[next] = num + sums[cur];
                    } else {
                        break;
                    }
                }
            }
        }
        dp[range - 1]
    }
}
