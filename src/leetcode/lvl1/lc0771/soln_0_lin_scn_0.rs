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
        let freqs_jwls: Vec<u8> = {
            let mut freqs: Vec<u8> = vec![0; 256];
            for ch in jewels.chars(){
                freqs[ch as usize] += 1;
            }
            freqs
        };
        let mut cnt: u8 = 0;
        for ch in stones.chars(){
            if freqs_jwls[ch as usize] > 0{
                cnt += 1;
            }
        }
        cnt as i32
    }
}