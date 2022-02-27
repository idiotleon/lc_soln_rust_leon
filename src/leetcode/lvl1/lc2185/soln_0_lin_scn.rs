/// @author: Leon
/// https://leetcode.com/problems/counting-words-with-a-given-prefix/
/// Time Complexity:    O(`_len_wds` * `len_p`)
/// Space Complexity:   O(`len_wd`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        let _len_wds: usize = words.len();
        let len_p: usize = pref.len();
        let chs_p: Vec<char> = pref.chars().collect();
        let mut cnt: i32 = 0;
        for word in words{
            let len_wd: usize = word.len();
            if len_wd < len_p{
                continue;
            }
            if Self::starts_with(&word.chars().collect(), &chs_p){
                cnt += 1;
            }
        }
        cnt
    }
    fn starts_with(chs_wd: &Vec<char>, chs_p: &Vec<char>) -> bool{
        for (idx, &ch) in chs_p.iter().enumerate(){
            if chs_wd[idx] != ch{
                return false;
            }
        }
        true
    }
}