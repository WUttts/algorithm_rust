///取巧解法，求最大上升区间
pub fn max_profit_1(prices: Vec<i32>) -> i32 {
    let mut res = 0;
    let (mut i, mut j) = (0, 1);

    while j < prices.len() {
        if prices[j] <= prices[j - 1] {
            res = res + prices[j - 1] - prices[i];
            i = j;
        }
        j += 1;
    }
    if i != j && prices[j - 1] > prices[i] {
        res = res + prices[j - 1] - prices[i];
    }
    res
}
///经典解法，动态规划
pub fn max_profit_2(prices: Vec<i32>) -> i32 {
    let len = prices.len();
    if len < 2 {
        return 0;
    }
    let mut dp = vec![vec![0; 2]; len];
    dp[0][0] = 0;
    dp[0][1] = -prices[0];

    for i in 1..len {
        //j = 0,为持有现金，所以比较的是，第一天持有现金的状态，和第一天持有股票但第二天卖出的状态相比
        dp[i][0] = dp[i - 1][0].max(dp[i - 1][1] + prices[i]);
        //同上，j=1,为持有股票，比较的是第一天持有股票的状态和第一天持有现金但第二天购入的状态相比。
        dp[i][1] = dp[i - 1][1].max(dp[i - 1][0] - prices[i]);
    }
    dp[len - 1][0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(7, max_profit_1(prices));
    }
    #[test]
    fn test_2() {
        let prices = vec![1, 2, 3, 4, 5];
        assert_eq!(4, max_profit_1(prices));
    }
}
