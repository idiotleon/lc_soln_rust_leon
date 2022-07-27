/// @author: Leon
/// https://leetcode.com/problems/best-poker-hand/
/// Time Complexity:    O(1)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
        let freqs_s: Vec<u8> = {
            let mut freqs: Vec<u8> = vec![0; 4];
            for suit in suits{
                let idx: usize = suit as usize - 'a' as usize;
                let freq = &mut freqs[idx];
                *freq += 1;
                if *freq == 5{
                    return "Flush".to_owned();
                }
            }
            freqs
        };
        let freqs_s: Vec<u8> = {
            let mut freqs: Vec<u8> = vec![0; 14];
            for rank in ranks{
                let idx: usize = rank as usize;
                let freq = &mut freqs[idx];
                *freq += 1;
                if *freq == 3{
                    return "Three of a Kind".to_owned();
                }
            }
            freqs
        };
        for freq in freqs_s{
            if freq == 2{
                return "Pair".to_owned();
            }
        }
        "High Card".to_owned()
    }
}