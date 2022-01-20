/// @author: Leon
/// https://leetcode.com/problems/divide-a-string-into-groups-of-size-k/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(1) / O(`len_s`)
/// Reference:
/// https://leetcode.com/problems/divide-a-string-into-groups-of-size-k/discuss/1696553/Rust-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn divide_string(mut s: String, k: i32, fill: char) -> Vec<String> {
        let len_s: usize = s.len();
        let ku: usize = k as usize;
        let rem: usize = len_s % ku;
        let parts: usize = len_s / ku + (rem > 0) as usize;
        let mut ans: Vec<String> = Vec::with_capacity(parts);
        (0..ku - rem).for_each(|_| s.push(fill));
        for idx in 0..parts {
            ans.push(s[idx * ku..(idx + 1) * ku].to_owned());
        }
        ans
    }
}
