impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        
        let mut r = x;
        let mut rev = 0;
        
        while r != 0 {
            let val = r % 10;
            rev = (rev * 10) + val;
            r /= 10;
        }
        
        rev == x
    }
}
