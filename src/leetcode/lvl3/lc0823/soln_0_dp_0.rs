use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/binary-trees-with-factors/
/// Time Complexity:    O(N)
/// Space Complexity:   O(N)
/// Reference:
/// https://leetcode.com/problems/binary-trees-with-factors/discuss/1107610/Rust-HashMap-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_factored_binary_trees(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1e9 as i64 + 7;
        let len_ns: usize = nums.len();
        let sorted: Vec<i32> = {
            let mut nums = nums;
            nums.sort_unstable();
            nums
        };
        let mut dp: HashMap<i32, i64> = sorted
            .iter()
            .map(|&n| (n, 1))
            .collect::<HashMap<i32, i64>>();
        for hi in 0..len_ns {
            for lo in 0..hi {
                if sorted[hi] % sorted[lo] == 0 {
                    let division = sorted[hi] / sorted[lo];
                    if let Some(&val) = dp.get(&division) {
                        if let Some(&val_lo) = dp.get(&sorted[lo]) {
                            if let Some(val_hi) = dp.get_mut(&sorted[hi]) {
                                *val_hi = (*val_hi + val_lo * val) % MOD;
                            }
                        }
                    }
                }
            }
        }
        return ((dp.values().sum::<i64>()) % MOD) as i32;
    }
}
