use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut ln = 0;
        let mut chrs = HashMap::new();
        let mut start = 0;
        let mut end = 0;
        
        for (i, c) in s.chars().enumerate() {
            if !chrs.contains_key(&c) {
                chrs.insert(c, i);
                end = i + 1;
            } else {
                if chrs[&c] + 1 > start {
                    start = chrs[&c] + 1;
                }
                chrs.insert(c, i);
                end = i + 1;
            }
            if end - start > ln {
                ln = end - start;
            }  
        }
        
        ln as i32
        
    }
}
