pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    if matrix.len() == 0 {
        return vec![];
    }

    let row_len = matrix.len();
    let col_len = matrix[0].len();
    // if col_len < 2 {
    //     return ;
    // }
    let mut res = vec![];

    let distance = row_len * col_len;

    let (mut upper, mut lower, mut left, mut right) = (0, row_len - 1, 0, col_len - 1);

    while res.len() < distance {
        if upper <= lower {
            for i in left..=right {
                res.push(matrix[upper][i]);
            }
            //上边界收缩
            upper += 1;
        }

        if left <= right {
            for i in upper..=lower {
                res.push(matrix[i][right]);
            }
            //右边界收缩
            if right > 0 {
                right -= 1;
            }
        }
        if upper < lower {
            let mut i = right;
            while i > left + 1 {
                res.push(matrix[lower][i]);
                i -= 1;
            }
            //下边界收缩
            if lower > 0 {
                lower -= 1;
            }
        }
        if left < right {
            let mut i = lower;
            while i > upper + 1 {
                res.push(matrix[i][left]);
                i -= 1;
            }
            //左边界收缩
            left += 1;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        // let matrix = vec![vec![1]];
        println!("{:?}", spiral_order(matrix));
    }
}
