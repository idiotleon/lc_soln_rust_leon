use std::collections::HashSet;
/// @author: Leon
/// https://leetcode.com/problems/jewels-and-stones/
/// Time Complexity:    O(`_len_sts`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let _len_jwls: usize = jewels.len();
        let _len_sts: usize = stones.len();
        let set_jwls: HashSet<char> = {
            let mut set: HashSet<char> = HashSet::new();
            for ch in jewels.chars(){
                set.insert(ch);
            }
            set 
        };
        let mut cnt: u8 = 0;
        for ch in stones.chars(){
            if set_jwls.contains(&ch){
                cnt += 1;
            }
        }
        cnt as i32
    }
}