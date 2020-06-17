impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        
        if divisor == 1 {
            return dividend;
        }
        if divisor == -1 {
            let tmp:i32 = 0;
            return tmp.checked_sub(dividend).unwrap_or(std::i32::MAX);
        }
        
        let sign = (dividend as u32 >> 31) ^ (divisor as u32 >> 31);
        
        // time to do binary division cause subtraction is too slow...
        let divid = (dividend as isize).abs();
        let mut divis = (divisor as isize).abs();
        let mut ans:i32 = 0;
        
        let mut bits = 31;
        let mut remainder = 0;
        while bits >= 0 {
            let bit = (divid & (1 << bits)) >> bits;
            remainder = (remainder << 1) + bit;
            if divis <= remainder {
                remainder = remainder - divis;
                ans = (ans << 1) + 1;
            } else {
                ans = ans << 1;
            }
            bits -= 1;
        }
        
        if sign == 1 {
            -ans
        } else {
            ans
        }
        
    }
}
