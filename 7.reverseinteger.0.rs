// probably a better way to do the overflow here

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        
        let mut x = x;
        let mut v = Vec::new();
        while x != 0 as i32 {
            v.push(x % 10);
            x /= 10;
        }
        v.reverse();
        let mut mult = 1;
        let mut total:i32 = 0;
        for i in v {
            let (res, overflow) = i.overflowing_mul(mult);
            if overflow {
                return 0;
            }
            let (res2, overflow) = total.overflowing_add(res); 
            if overflow {
                return 0;
            }
            total = res2;
            mult *= 10;
            
        }
        
        total
    }
}

// 1_534_236_469   1534236469
// 1_056_389_759

