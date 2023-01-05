    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let num_rows = num_rows as usize;
        let mut res = vec![vec![]; num_rows];
        for i in 0..num_rows {
            let mut rows = vec![0; i + 1];
            for j in 0..rows.len() {
                if j == 0 || j == rows.len() - 1 {
                    rows[j] = 1;
                    continue;
                }
                rows[j] = res[i - 1][j] + res[i - 1][j - 1];
            }
            res[i] = rows;
        }
    
        res
    }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let num_rows = 6;
        let res = generate(num_rows);
        println!("{:?}", res);
    }
}
