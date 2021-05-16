/// https://leetcode.com/problems/valid-number/
/// 
/// Time Complexity:    O(len_s)
/// Space Complexity:   O(len_s)
/// 
/// Reference:
/// https://leetcode.com/problems/valid-number/discuss/23738/Clear-Java-solution-with-ifs/937169
/// https://leetcode.com/problems/valid-number/discuss/23738/Clear-Java-solution-with-ifs/22978
/// https://leetcode.com/problems/valid-number/discuss/23738/Clear-Java-solution-with-ifs
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_number(s: String) -> bool {
        let chs: Vec<char> = s.trim().chars().collect();
        
        let mut dot_seen:bool = false;
        let mut e_seen:bool = false;
        let mut num_seen:bool = false;
        
        for (idx, &ch) in chs.iter().enumerate(){
            match ch {
                '0'..='9' => {
                    num_seen = true;
                },
                '.' => {
                    if e_seen || dot_seen{
                        return false;
                    }
                    
                    dot_seen = true;
                },
                'e' | 'E' => {
                    if e_seen || !num_seen{
                        return false;
                    }
                    
                    num_seen = false;
                    e_seen = true;
                },
                '-' | '+' => {
                    if idx != 0 && chs[idx - 1] != 'e'{
                        return false;
                    }
                },
                _ => return false,
            }
        }
        
        num_seen
    }
}