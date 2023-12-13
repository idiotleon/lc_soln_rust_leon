use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/rank-transform-of-an-array/
/// Time Complexity:    O(`len_ns` * lg(`len_ns`))
/// Space Complexity:   O(`len_ns`)
/// Reference:
/// https://leetcode.com/problems/rank-transform-of-an-array/solutions/489753/java-c-python-hashmap/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn array_rank_transform(nums: Vec<i32>) -> Vec<i32> {
        let len_ns: usize = nums.len();
        let sorted: Vec<i32> = {
            let mut res = nums.to_vec();
            res.sort();
            res
        };
        let num_to_rank: HashMap<i32, i32> = {
            let mut map: HashMap<i32, i32> = HashMap::with_capacity(len_ns);
            let mut rank: i32 = 1;
            for num in sorted {
                if !map.contains_key(&num) {
                    map.insert(num, rank);
                    rank += 1;
                }
            }
            map
        };
        let mut ans: Vec<i32> = Vec::with_capacity(len_ns);
        for num in nums {
            if let Some(&rank) = num_to_rank.get(&num) {
                ans.push(rank);
            }
        }
        return ans;
    }
}
