/// @author: Leon
/// https://leetcode.com/problems/repeated-string-match/
/// Time Complexity:    O(`len1` * `len2`)
/// Space Complexity:   O(max(`len1`, `len2`))
/// Reference:
/// https://leetcode.com/problems/repeated-string-match/discuss/108084/C%2B%2B-4-lines-O(m-*-n)-or-O(1)-and-KMP-O(m-%2B-n)-or-O(n)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn repeated_string_match(str1: String, str2: String) -> i32 {
        let len1: usize = str1.len();
        let chs1: Vec<char> = str1.chars().collect();
        let len2: usize = str2.len();
        let chs2: Vec<char> = str2.chars().collect();
        for idx1 in 0..len1{
            let mut idx2: usize = 0;
            while idx2 < len2{
                if chs1[(idx1 + idx2) % len1] == chs2[idx2]{
                    idx2 += 1;
                }else{
                    break;
                }
            }
            if idx2 == len2{
                return ((idx1 + idx2 - 1) / len1) as i32 + 1;
            }
        }
        -1
    }
}