/// https://leetcode.com/problems/reverse-words-in-a-string-ii/
/// 
/// Time Complexity:    O(`len_chs`)
/// Space Complexity:   O(1)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reverse_words(chs: &mut Vec<char>) {
        let mut chs = chs;
        const CH_SPACE: char = ' ';
        let len_chs = chs.len();
        Self::reverse(0, len_chs - 1, &mut chs);
        
        let mut lo: usize = 0;
        let mut hi: usize = 0;
        
        while hi < len_chs{
            while hi < len_chs && chs[hi] != CH_SPACE{
                hi += 1;
            }
            
            Self::reverse(lo, hi - 1, &mut chs);
            
            lo = hi + 1;
            hi += 1;
        }
    }
    
    fn reverse(mut lo: usize, mut hi: usize, chs: &mut Vec<char>){
        while lo < hi{
            chs.swap(lo, hi);
            lo += 1;
            hi -= 1;
        }
    }
}