pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
    let (r, c) = (r as usize, c as usize);
    let (m, n) = (mat.len(), mat[0].len());
    if r * c != m * n {
        return mat;
    }
    let mut res = vec![vec![0; c]; r];
    for i in 0..r * c {
        res[i / c][i % c] = mat[i / n][i % n];
    }
    res
}
