// 算法可以直接简化为只要今天比昨天大，就卖出，同时也可以买进（但不知道明天是否会涨呢？）。
// 上帝视角： 扫描一遍 只要后一天比前一天大 就把这两天的差值加一下

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut ret = 0;
    for i in 0 .. prices.len() - 1 {
        if prices[i+1] > prices[i] { ret += prices[i+1] - prices[i] }
    }
    ret
}

fn main() {
    println!("{:?}", max_profit(vec![7,1,5,3,6,4]));
}
