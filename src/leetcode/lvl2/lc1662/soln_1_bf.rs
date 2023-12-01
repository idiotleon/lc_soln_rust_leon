struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn array_strings_are_equal(words1: Vec<String>, words2: Vec<String>) -> bool {
        let len_ws1: usize = words1.len();
        let len_ws2: usize = words2.len();
        let s1: String = words1.join("");
        let s2: String = words2.join("");
        return s1 == s2;
    }
}