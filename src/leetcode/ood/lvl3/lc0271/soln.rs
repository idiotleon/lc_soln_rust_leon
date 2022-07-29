/// @author: Leon
/// https://leetcode.com/problems/encode-and-decode-strings/
/// Time Complexities:
///         `encode()`: O(`_len_ss` * avg_len_s)
///         `decode()`: O(`len_cs`)
/// Space Complexity:   O(`len_cs`)
/// Reference:
/// https://leetcode.com/problems/encode-and-decode-strings/discuss/964553/Chunked-Transfer-Encoding-in-Rust
#[allow(dead_code)]
struct Codec {}

#[allow(dead_code)]
impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn encode(&self, strs: Vec<String>) -> String {
        let _len_ss: usize = strs.len();
        let mut ans: String = "".to_owned();
        for s in strs {
            let chunk_header = std::char::from_u32(s.len() as u32).unwrap();
            ans.push(chunk_header);
            ans.push_str(&s);
        }
        ans
    }

    fn decode(&self, s: String) -> Vec<String> {
        let chs: Vec<char> = s.chars().collect();
        // please be reminded that the length here should be the length of char vector,
        // instead of the string itself, of which one character might cost more than one bytes.
        let len_cs: usize = chs.len();
        let mut ans: Vec<String> = Vec::new();
        let mut idx: usize = 0;
        while idx < len_cs {
            let len = chs[idx] as usize;
            let chunk = chs[(idx + 1)..=(idx + len)].iter().collect();
            ans.push(chunk);
            idx += 1 + len;
        }
        ans
    }
}
