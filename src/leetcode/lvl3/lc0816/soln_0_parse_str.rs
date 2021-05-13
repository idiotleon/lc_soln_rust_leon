/// https://leetcode.com/problems/ambiguous-coordinates/
/// 
/// Time Complexity:    O()
/// Space Complexity:   O()
/// 
/// Reference:
/// https://leetcode.com/problems/ambiguous-coordinates/discuss/123875/Really-clear-Java-code
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn ambiguous_coordinates(s: String) -> Vec<String> {
        let len_s: usize = s.len();
        let slice: &str = &s[1..len_s - 1];
        let len_slice: usize = slice.len();

        let mut ans: Vec<String> = Vec::new();
        
        for i in 1..len_slice{
            let left_strs: Vec<String> = Self::allowed(&slice[..i]);
            let right_strs: Vec<String> = Self::allowed(&slice[i..]);
            
            for left_str in left_strs{
                for right_str in right_strs.iter(){
                    ans.push(format!("({}, {})", left_str, right_str));
                }
            }
        }
        
        ans
    }
    
    fn allowed(slice: &str) -> Vec<String>{
        let len_s = slice.len();
        let chs: Vec<char> = slice.chars().collect();
        
        let mut candidates: Vec<String> = Vec::new();
        
        if chs[0] == '0' && chs[len_s - 1] == '0'{
            if len_s == 1{
                candidates.push("0".to_string());
            }
            return candidates;
        }
        
        if chs[0] == '0'{
            candidates.push(format!("0.{}", &slice[1..]));
            return candidates;
        }
        
        if chs[len_s - 1] == '0'{
            candidates.push(slice.to_string());
            return candidates;
        }
        
        candidates.push(slice.to_string());
        for i in 1..len_s{
            candidates.push(format!("{}.{}", &slice[..i], &slice[i..]));
        }
        
        candidates
    }
}