#[allow(unused)]
struct NumMatrix {
    sums: Vec<Vec<i32>>,
}

#[allow(unused)]
impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut pre_sum = vec![vec![0; n + 1]; m + 1];

        for i in 0..=m {
            for j in 1..=n {
                pre_sum[i][j] = pre_sum[i - 1][j] + pre_sum[i][j - 1] + matrix[i - 1][j - 1]
                    - pre_sum[i - 1][j - 1];
            }
        }

        Self { sums: pre_sum }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let (row1, col1, row2, col2) = (row1 as usize, col1 as usize, row2 as usize, col2 as usize);
        return self.sums[row2 + 1][col2 + 1]
            - self.sums[row1][col2 + 1]
            - self.sums[row2 + 1][col1]
            + self.sums[row1][col1];
    }
}