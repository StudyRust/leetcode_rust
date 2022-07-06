// 超时间了。。。
// pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
//     use std::collections::HashSet;
//     let mut set = HashSet::new();
//     for (i, ii) in digits.iter().enumerate() {
//         if *ii == 0 {
//             continue;
//         }
//         for (j, jj) in digits.iter().enumerate() {
//             if j == i {
//                 continue;
//             }
//             for (k, kk) in digits.iter().enumerate() {
//                 if k == j || k == i {
//                     continue;
//                 }
//                 if kk % 2 == 0 {
//                     let tmp = vec!(
//                         ii.to_string(),
//                         jj.to_string(),
//                         kk.to_string(),
//                     ).join("");
//                     set.insert(tmp.parse::<i32>().unwrap());
//                 }
//             }
//         }
//     }
//     let mut ret: Vec<i32> = set.into_iter().collect();
//     ret.sort();
//     ret
// }

pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    for (i, ii) in digits.iter().enumerate() {
        if *ii == 0 {
            continue;
        }
        for (j, jj) in digits.iter().enumerate() {
            if j == i {
                continue;
            }
            for (k, kk) in digits.iter().enumerate() {
                if k == j || k == i {
                    continue;
                }
                if kk % 2 == 0 {
                    // 优化此行后通过，数值运算比字符串运算快。。
                    set.insert(ii * 100 + jj * 10 + kk);
                }
            }
        }
    }
    let mut ret: Vec<i32> = set.into_iter().collect();
    ret.sort();
    ret
}

fn main() {
    println!("{:?}", find_even_numbers(vec![0,7,0,7,3,3,9,0,7,0,6,9,8,3,1,3,8,0,9,6,0,3,2,4,5,9,0,4,7,6,7,2,7,0,3,7,3,0,3,7,8,2,6,4,7,4,8,5,5,6,7,6,6,1,3,0,1,6,0,9,2,0,8,7,9,4,0,6,4,0]));
}
