pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
    let mut sorted_nums = nums.clone();
    sorted_nums.sort();
    let mut min = std::i32::MAX;
    for i in 0..=sorted_nums.len() - k as usize {
        let minus = sorted_nums[i + k as usize - 1] - sorted_nums[i];
        if minus < min { min = minus }
    }
    min
}

fn main() {
    let nums = vec![41900,69441,94407,37498,20299,10856,36221,2231,54526,79072,84309,76765,92282,13401,44698,17586,98455,47895,98889,65298,32271,23801,83153,12186,7453,79460,67209,54576,87785,47738,40750,31265,77990,93502,50364,75098,11712,80013,24193,35209,56300,85735,3590,24858,6780,50086,87549,7413,90444,12284,44970,39274,81201,43353,75808,14508,17389,10313,90055,43102,18659,20802,70315,48843,12273,78876,36638,17051,20478];
    let k = 5;
    println!("{}", minimum_difference(nums, k));
    let nums = vec![90];
    let k = 1;
    println!("{}", minimum_difference(nums, k));
    let nums = vec![9,4,1,7];
    let k = 2;
    println!("{}", minimum_difference(nums, k));
}
