/// https://leetcode.com/problems/unique-binary-search-trees/
/// Time Complexity:    O(`n` ^ 1)
/// Space Complexity:   O(`n`)
/// `dp[k]`, the number of BSTs built from 1...k
/// to build a tree, one needs to pick a `root` node,
/// and know how many possible left and right subtrees can be built under that node.
/// The result is to multiply them.
/// References:
/// https://leetcode.com/problems/unique-binary-search-trees/discuss/31707/Fantastic-Clean-Java-DP-Solution-with-Detail-Explaination
/// https://leetcode.com/problems/unique-binary-search-trees/discuss/703488/Detailed-Explanation-%3A-Mental-Leap-on-Why-the-approach-actually-worksd
/// https://leetcode-cn.com/problems/unique-binary-search-trees/solution/shou-hua-tu-jie-san-chong-xie-fa-dp-di-gui-ji-yi-h/
/// Catalan number:
/// https://en.wikipedia.org/wiki/Catalan_number
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n: usize = n as usize;
        let dp = {
            let mut dp: Vec<i32> = vec![0; n + 1];
            dp[0] = 1;
            dp[1] = 1;
            for lvl in 2..=n {
                for rt in 1..=lvl {
                    dp[lvl] += dp[lvl - rt] * dp[rt - 1];
                }
            }
            dp
        };
        dp[n]
    }
}
