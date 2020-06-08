// could be improved by always searching for same char left/right
// then search from that new middle each time.
// also duplicates code unecessarily

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut res_start = 0;
        let mut res_end = 0;
        let res = 0;
        
        for (i,c) in s.chars().enumerate() {
            let mut pal = true;
            
            let mut start = i;
            let mut end = i;
            
            while pal {
                if start != 0 && end < s.len() -1 {
                    let left = s.as_bytes()[start - 1];
                    let right =  s.as_bytes()[end + 1];
                    if left == right {
                        start -= 1;
                        end += 1;
                    } else {
                        pal = false;
                    }
                } else {
                    pal = false;
                }
            }
            
            if (res_end - res_start) <= (end - start) {
                res_end = end + 1;
                res_start = start;
            }
            
            if i < s.len() - 1 &&  c == s.as_bytes()[i + 1] as char {
                start = i;
                end = i + 1;
                pal = true;
                while pal {
                    if start != 0 && end < s.len() -1 {
                        let left = s.as_bytes()[start - 1];
                        let right =  s.as_bytes()[end + 1];
                        if left == right {
                            start -= 1;
                            end += 1;
                        } else {
                            pal = false;
                        }
                    } else {
                        pal = false;
                    }
                }
            }
            
            if (res_end - res_start) <= (end - start) {
                res_end = end + 1;
                res_start = start;
            }
        }
        
        s[res_start..res_end].to_string()
         
    }
}
