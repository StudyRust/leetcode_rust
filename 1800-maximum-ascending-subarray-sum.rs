pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
    let mut arr: Vec<i32> = Vec::new();
    let mut tmp_arr = Vec::new();
    let mut pre_num = -1;
    for num in nums {
        if num > pre_num {
            tmp_arr.push(num);
        }
        if num <= pre_num {
            arr.push(tmp_arr.iter().sum());
            tmp_arr = vec!(num);
        }
        pre_num = num;
    }
    arr.push(tmp_arr.iter().sum());
    arr.sort();
    *arr.last().unwrap()
}

fn main() {
    println!("{:?}", max_ascending_sum(vec![10,20,30,5,10,50]));
}