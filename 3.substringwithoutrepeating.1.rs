use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut ln = 0;
        let mut chrs = HashMap::new();
        let mut start = 0;
   
        for (i, c) in s.chars().enumerate() {
            if chrs.contains_key(&c) {
                let x = chrs[&c] + 1;
                if x  > start {
                    start = x;
                }
            }
            chrs.insert(c, i);
            
            if (i + 1) - start > ln {
                ln = (i+1) - start;
            }  
        }
        
        ln as i32
        
    }
}
