/// https://leetcode.com/problems/word-ladder/
/// 
/// Time Complexity:    O()
/// Space Complexity:   O()
/// 
/// Reference:
/// https://leetcode.com/problems/word-ladder/discuss/40711/Two-end-BFS-in-Java-31ms.
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        use std::collections::HashSet;
        
        // not used
        // let len_s: usize = begin_word.len();
        let word_set: HashSet<String> = word_list.into_iter().collect();
        if !word_set.contains(&end_word.to_string()){
            return 0
        }
        
        let mut begin_set: HashSet<String> = HashSet::new();
        begin_set.insert(begin_word.to_string());
        let mut end_set: HashSet<String> = HashSet::new();
        end_set.insert(end_word.to_string());
        
        let mut step = 1;
        let mut used: HashSet<String> = HashSet::new();
        
        while !begin_set.is_empty() && !end_set.is_empty(){
            if begin_set.len() > end_set.len(){
                let tmp_set = begin_set;
                begin_set = end_set;
                end_set = tmp_set;
            }
            
            let mut temp: HashSet<String> = HashSet::new();
            for word in begin_set.iter(){
                let mut chs: Vec<char> = word.chars().collect();
                
                for idx in 0..chs.len() {
                    for ch in (b'a'..=b'z').map(char::from).collect::<Vec<_>>() {
                        let hold: char = chs[idx];
                        chs[idx] = ch;
                        let target: String = chs.iter().collect();
                        
                        if end_set.contains(&target.to_string()){
                            return step + 1;
                        }
                        
                        if word_set.contains(&target.to_string()) && used.insert(target.to_string()){
                            temp.insert(target.to_string());
                        }
                        chs[idx] = hold;
                    }
                }
            }
            begin_set = temp;
            step += 1;
        }
        
        0
    }
}