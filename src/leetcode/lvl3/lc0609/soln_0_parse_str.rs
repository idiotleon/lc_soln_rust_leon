/// @author: Leon
/// https://leetcode.com/problems/find-duplicate-file-in-system/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/find-duplicate-file-in-system/discuss/1215511/Rust-HashMap-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        let mut file_to_freq = HashMap::<&str, Vec<String>>::new();
        for path in &paths {
            let files = path.split(' ').collect::<Vec<_>>();
            for &file in &files[1..] {
                if let Some(pos) = file.chars().position(|c| c == '(') {
                    file_to_freq
                        .entry(&file[pos + 1..file.len() - 1])
                        .or_insert_with(Vec::new)
                        .push(String::new() + files[0] + "/" + &file[..pos]);
                }
            }
        }
        file_to_freq
            .into_iter()
            .filter_map(|(_, v)| if v.len() > 1 { Some(v) } else { None })
            .collect()
    }
}
