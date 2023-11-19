use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/beautiful-array/
/// Time Complexity:    O(`n` * lg(`n`))
/// Space Complexity:   O(`n` * lg(`n`))
/// Reference:
/// https://leetcode.com/problems/beautiful-array/editorial/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn beautiful_array(n: i32) -> Vec<i32> {
        let mut memo: HashMap<i32, Vec<i32>> = HashMap::with_capacity(n as usize);
        return Self::dfs(n, &mut memo);
    }
    fn dfs(n: i32, memo: &mut HashMap<i32, Vec<i32>>) -> Vec<i32> {
        if let Some(m) = memo.get(&n) {
            return m.to_vec();
        }
        let mut res: Vec<i32> = vec![0; n as usize];
        if n == 1 {
            res[0] = 1;
        } else {
            let mut t: usize = 0;
            for x in Self::dfs((n + 1) / 2, memo) {
                res[t] = 2 * x - 1;
                t += 1;
            }
            for x in Self::dfs(n / 2, memo) {
                res[t] = 2 * x;
                t += 1;
            }
        }
        memo.insert(n, res.to_vec());
        return res;
    }
}
