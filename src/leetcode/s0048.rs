///###[48. 旋转图像](https://leetcode.cn/problems/rotate-image/)
/// 思路是按对主角线对折，再调换就完成顺时针旋转了
pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let len = matrix.len();
    for i in 0..len {
        for j in i..len {
            if i == j {
                continue;
            }
            let temp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = temp;
        }
    }

    // let temp = matrix[i][j];
    // matrix[i][j] = matrix[i][len - i - 1];
    // matrix[i][len - i - 1] = temp;

    for item in matrix {
        let mut l = 0;
        let mut r = len - 1;
        while l < r {
            let temp = item[l];
            item[l] = item[r];
            item[r] = temp;
            l += 1;
            r -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        rotate(&mut matrix);

        println!("{:?}", matrix);
    }
}
