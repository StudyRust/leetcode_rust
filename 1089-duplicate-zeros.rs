// 从后往前冒泡交换，然后将最后一个元素冒泡到0后面一位，然后将它变为0
// 因为最后一个元素是要被挤出去的那一个

pub fn duplicate_zeros(arr: &mut Vec<i32>) {
    let mut i = 0;
    while i < arr.len() {
        if arr[i] == 0 {
            for j in (i+1..arr.len()).rev() {
                let tmp = arr[j];
                arr[j] = arr[j-1];
                arr[j-1] = tmp;
            }
            arr[i] = 0;
            i+=1;
        }
        i+=1;
    }
}

fn main() {
    let mut arr = vec![1,0,2,3,0,4,5,0];
    duplicate_zeros(&mut arr);
    println!("{:?}", arr);
}
