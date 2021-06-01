/// https://leetcode.com/problems/maximum-value-after-insertion/
///
/// Time Complexity:    O(L)
/// Space Complexity:   O(L)
///
/// Reference:
/// https://leetcode.com/problems/maximum-value-after-insertion/discuss/1239779/Java-Simple/955120
/// https://leetcode.com/problems/maximum-value-after-insertion/discuss/1239779/Java-Simple
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_value(n: String, x: i32) -> String {
        let len_s: usize = n.len();
        let chs: Vec<char> = n.chars().collect();
        let is_negative: bool = chs[0] == '-';

        for (idx, ch) in chs.into_iter().enumerate() {
            if !is_negative && ((ch as i32 - '0' as i32) < x)
                || is_negative && ((ch as i32 - '0' as i32) > x)
            {
                return format!(
                    "{}{}{}",
                    n[..idx].to_string(),
                    x.to_string(),
                    n[idx..].to_string()
                );
            }
        }
        format!("{}{}", n, x.to_string())
    }
}
