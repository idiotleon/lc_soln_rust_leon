use std::collections::BTreeMap;

/// @author: Leon
/// https://leetcode.com/problems/the-number-of-beautiful-subsets/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// This is NOT yet a correct solution.
/// Reference:
/// https://leetcode.com/problems/the-number-of-beautiful-subsets/solutions/3363862/c-java-python-evolve-brute-force-to-dp-explained-7-approaches/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        let _len_ns: usize = nums.len();
        let modu_to_freq: BTreeMap<i32, BTreeMap<i32, u8>> = {
            let mut map: BTreeMap<i32, BTreeMap<i32, u8>> = BTreeMap::new();
            for num in nums {
                let modu = num % k;
                *map.entry(modu).or_default().entry(num).or_default() += 1;
            }
            map
        };
        let mut ans: i32 = 1;
        for (_modu, num_to_freq) in modu_to_freq.into_iter() {
            let mut prev_num: i32 = -k;
            let mut prev2 = 0;
            let mut prev1 = 1;
            let mut cur = 1;
            for (num, freq) in num_to_freq.into_iter() {
                let skip = prev1;
                let take = ((1 << freq) - 1) * (num - if prev_num == k { prev2 } else { prev1 });
                cur = skip + take;
                prev2 = prev1;
                prev1 = cur;
                prev_num = num;
            }
            ans *= cur;
        }
        return ans - 1;
    }
}
