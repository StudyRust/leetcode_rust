pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut ret = vec![];
    for i in 0..img.len() {
        let mut tmp = vec![];
        let mut tmp_sum = 0;
        let mut tmp_count = 0;
        for j in 0..img[0].len() {
            {
                {
                    tmp_sum += img[i][j];
                    tmp_count += 1
                }
                if j > 0 {
                    tmp_sum += img[i][j-1];
                    tmp_count += 1
                }
                if j + 1 < img[0].len() {
                    tmp_sum += img[i][j+1];
                    tmp_count += 1
                }
            }
            if i > 0 {
                {
                    tmp_sum += img[i-1][j];
                    tmp_count += 1
                }
                if j > 0 {
                    tmp_sum += img[i-1][j-1];
                    tmp_count += 1
                }
                if j + 1 < img[0].len() {
                    tmp_sum += img[i-1][j+1];
                    tmp_count += 1
                }
            }
            if i + 1 < img.len() {
                {
                    tmp_sum += img[i+1][j];
                    tmp_count += 1
                }
                if j > 0 {
                    tmp_sum += img[i+1][j-1];
                    tmp_count += 1
                }
                if j + 1 < img[0].len() {
                    tmp_sum += img[i+1][j+1];
                    tmp_count += 1
                }
            }
            tmp.push((tmp_sum as f64 / tmp_count as f64).floor() as i32);
            tmp_sum = 0;
            tmp_count = 0;
        }
        ret.push(tmp);
    }
    ret
}

fn main() {
    let img = vec![vec![1,1,1], vec![1,0,1], vec![1,1,1]];
    println!("{:?}", image_smoother(img));

    let img = vec![vec![2,3,4], vec![5,6,7], vec![8,9,10], vec![11,12,13], vec![14,15,16]];
    println!("{:?}", image_smoother(img));
}
