// the lookup table is probably not ideal but seems fast...

use std::collections::HashMap;

impl Solution {
    
    pub fn my_atoi(str: String) -> i32 {
        let atoi:HashMap<char, i32> = [('1', 1), ('2', 2), ('3', 3), ('4', 4),('5', 5),
                                    ('6', 6), ('7', 7), ('8', 8), ('9', 9), ('0', 0),
                                    ]
                                    .iter().cloned().collect();
        
        let mut result:i32 = 0;
        let mut nums = Vec::new();
        let mut neg = false;
        let mut hitnumber = false;
        for c in str.chars() {
            if c == ' ' && !hitnumber {
                continue;
            }
            if (c == '+' || c == '-') && !hitnumber {
                hitnumber = true;            
                if c == '-' {
                    neg = true;
                }
                continue;
            }
            else if atoi.contains_key(&c) {
                nums.push(atoi[&c]);
                hitnumber = true;
            } else {
                break;
            }
        }
        
        nums.reverse();
        
        let mut mult = if neg {
            -1
        } else {
            1
        };
        for n in nums {
            result = result.saturating_add(n.saturating_mul(mult));
            mult = mult.saturating_mul(10);
        }
        
    
        result
        
    }
}
