/// @author: Leon
/// https://leetcode.com/problems/ways-to-split-array-into-three-subarrays/
/// Time Complexity:    O(`len_ns` * lg(`len_ns`))
/// Space Complexity:   O(`len_ns`)
/// Reference:
/// https://leetcode.com/problems/ways-to-split-array-into-three-subarrays/discuss/999113/JavaScala-Detailed-Explanation-Prefix-Sum-Binary-Search
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn ways_to_split(nums: Vec<i32>) -> i32 {
        const MOD: i32 = 1e9 as i32 + 7;
        let len_ns: usize = nums.len();
        let prefix_sums: Vec<i32> = {
            let mut prefix_sums: Vec<i32> = vec![0; len_ns];
            prefix_sums[0] = nums[0];
            for idx in 1..len_ns {
                prefix_sums[idx] = prefix_sums[idx - 1] + nums[idx];
            }
            prefix_sums
        };
        let mut cnt: i32 = 0;
        for idx in 1..len_ns - 1 {
            if prefix_sums[idx - 1] > (prefix_sums[len_ns - 1] - prefix_sums[idx - 1]) / 2 {
                break;
            }
            let lo = Self::guess(idx as isize, &prefix_sums, prefix_sums[idx - 1], true);
            let hi = Self::guess(idx as isize, &prefix_sums, prefix_sums[idx - 1], false);
            if lo == -1 || hi == -1 {
                continue;
            }
            cnt = (cnt + (hi - lo + 1) % MOD) % MOD;
        }
        cnt
    }
    fn guess(idx: isize, prefix_sums: &Vec<i32>, sum_left: i32, flag_search_left: bool) -> i32 {
        let len_ns: isize = prefix_sums.len() as isize;
        let mut lo: isize = idx;
        let mut hi: isize = len_ns - 2;
        let mut cnt: i32 = -1;
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            let sum_mid: i32 = prefix_sums[mid as usize] - prefix_sums[idx as usize - 1];
            let sum_right: i32 = prefix_sums[len_ns as usize - 1] - prefix_sums[mid as usize];
            if sum_left <= sum_mid && sum_mid <= sum_right {
                cnt = mid as i32;
                if flag_search_left {
                    hi = mid - 1;
                } else {
                    lo = mid + 1;
                }
            } else if sum_left > sum_mid {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        cnt
    }
}
