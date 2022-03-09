/// @author: Leon
/// https://leetcode.com/problems/strobogrammatic-number-ii/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/strobogrammatic-number-ii/discuss/67280/AC-clean-Java-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_strobogrammatic(n: i32) -> Vec<String> {
        Self::dfs(n, n)
    }
    fn dfs(n: i32, m: i32) -> Vec<String> {
        if n == 0 {
            return vec!["".to_owned()];
        }
        if n == 1 {
            return vec!["0".to_owned(), "1".to_owned(), "8".to_owned()];
        }
        let strs = Self::dfs(n - 2, m);
        let len_s: usize = strs.len();
        let ans: Vec<String> = {
            let mut res = Vec::with_capacity(len_s);
            for str in strs {
                if n != m {
                    res.push(format!("0{}0", str));
                }
                res.push(format!("1{}1", str));
                res.push(format!("6{}9", str));
                res.push(format!("8{}8", str));
                res.push(format!("9{}6", str));
            }
            res
        };
        ans
    }
}
