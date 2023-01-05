#[allow(unused)]
///实现一个计算器
/// 给定一个包含正整数、加(+)、减(-)、乘(*)、除(/)的算数表达式(括号除外)，计算其结果。
///**表达式仅包含非负整数**，+， - ，*，/ 四种运算符和空格  。 整数除法仅保留整数部分。
pub fn calculate(s: String) -> i32 {
    //先乘除后加减
    let s = s.trim();

    0
}
#[allow(unused)]
fn eval(operator: Operator) -> i32 {
    0
}
#[allow(unused)]
enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}
