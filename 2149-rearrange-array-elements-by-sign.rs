pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
    let mut ret = vec!(0; nums.len());
    let mut tmp_pos_index = 0;
    let mut tmp_neg_index = 0;
    for n in nums {
        if n > 0 {
            ret[tmp_pos_index*2] = n;
            tmp_pos_index += 1;
        } else {
            ret[tmp_neg_index*2+1] = n;
            tmp_neg_index += 1;
        }
    }
    ret
}

fn main() {
    let nums = vec![3,1,-2,-5,2,-4];
    println!("{:?}", rearrange_array(nums));
}
