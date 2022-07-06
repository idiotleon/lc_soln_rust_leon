/// @author: Leon
/// https://leetcode.com/problems/split-array-into-fibonacci-sequence/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/split-array-into-fibonacci-sequence/discuss/133936/short-and-fast-backtracking-solution
/// https://leetcode.com/problems/split-array-into-fibonacci-sequence/discuss/334250/Short-4ms-C%2B%2B-solution-beats-84
/// https://leetcode.com/problems/split-array-into-fibonacci-sequence/discuss/139690/Logical-Thinking-with-Clear-Java-Code
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn split_into_fibonacci(num: String) -> Vec<i32> {
        let len_s: usize = num.len();
        let chs: Vec<char> = num.chars().collect();
        let mut ans: Vec<i32> = Vec::with_capacity(len_s);
        Self::dfs(0, &chs, &mut ans);
        ans
    }
    fn dfs(idx_start: usize, chs: &Vec<char>, res: &mut Vec<i32>) -> bool {
        let len_s: usize = chs.len();
        if idx_start == len_s && res.len() >= 3 {
            return true;
        }
        let mut num: i64 = 0_i64;
        for idx in idx_start..len_s {
            num = num * 10 + chs[idx] as i64 - '0' as i64;
            if num > i32::MAX as i64 {
                return false;
            }
            if chs[idx_start] == '0' && idx > idx_start {
                return false;
            }
            if res.len() < 2 || num == (res[res.len() - 1] as i64 + res[res.len() - 2] as i64) {
                res.push(num as i32);
                if Self::dfs(1 + idx, chs, res) {
                    return true;
                }
                res.pop();
            }
        }
        false
    }
}
