/// @author: Leon
/// https://leetcode.com/problems/sorting-the-sentence/
/// Time Complexity:    O(`_len_s` + `_len_str` * lg(`_len_strs`))
/// Space Complexity:   O(`_len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let _len_s: usize = s.len();
        const SPACE: char = ' ';
        let strs: Vec<&str> = {
            let mut strs: Vec<&str> = s.split(SPACE).collect();
            strs.sort_by(|a, b| {
                a[a.len() - 1..]
                    .parse::<i32>()
                    .unwrap()
                    .cmp(&b[b.len() - 1..].parse::<i32>().unwrap())
            });
            strs
        };
        let _len_strs: usize = strs.len();
        strs.into_iter()
            .map(|s| s[..s.len() - 1].to_owned())
            .collect::<Vec<String>>()
            .join(&SPACE.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let s = "is2 sentence4 This1 a3".to_owned();
        let actual = Solution::sort_sentence(s);
        let expected = "This is a sentence".to_owned();
        assert_eq!(expected, actual);
    }
}
