pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
    let mut ret = vec![];
    'outer: for (i, n) in prices[0..prices.len()].iter().enumerate() {
        for j in (i+1)..prices.len() {
            if prices[j] <= *n {
                ret.push(n - prices[j]);
                continue 'outer;
            }
        }
        ret.push(*n);
    }
    ret
}

fn main() {
    println!("{:?}", final_prices(vec![8,4,6,2,3]));
    println!("{:?}", final_prices(vec![1,2,3,4,5]));
    println!("{:?}", final_prices(vec![10,1,1,6]));
}
