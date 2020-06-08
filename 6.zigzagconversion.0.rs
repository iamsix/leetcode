impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        
        
        let mut rows: Vec<Vec<char>> = Vec::new();
        for _ in 0..num_rows {
            rows.push(Vec::new())
        }
        
        let mut cur_row = 0;
        let mut down = true;
        for c in s.chars() {
            rows[cur_row].push(c);
            if cur_row == num_rows as usize -1 {
                down = false;
            } else if cur_row == 0 {
                down = true;
            }
            if down {
                cur_row += 1;
            } else {
                cur_row -= 1;
            }
        }
        
        
        let mut out = Vec::new();
        for row in rows {
            out.extend(row);
        }
        out.iter().collect()
    }
}
