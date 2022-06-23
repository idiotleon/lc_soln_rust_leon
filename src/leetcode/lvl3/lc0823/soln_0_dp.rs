use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/binary-trees-with-factors/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/binary-trees-with-factors/discuss/1107610/Rust-HashMap-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_factored_binary_trees(arr: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut dp = arr.iter().map(|&n| (n, 1)).collect::<HashMap<_, _>>();
        let mut arr = arr;
        arr.sort_unstable();
        for hi in 0..arr.len() {
            for lo in 0..hi {
                if arr[hi] % arr[lo] == 0 {
                    if let Some(&val) = dp.get(&(arr[hi] / arr[lo])) {
                        let val_j = *dp.get_mut(&arr[lo]).unwrap();
                        if let Some(val_i) = dp.get_mut(&arr[hi]) {
                            *val_i = (*val_i + val_j * val) % MOD;
                        }
                    }
                }
            }
        }
        (dp.values().sum::<i64>() % MOD) as i32
    }
}
