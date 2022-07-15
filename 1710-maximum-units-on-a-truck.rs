// box_types.sort{|a, b|b[1]<=>a[1]}.map{|e|[e[1]]*e[0]}.flatten[...truck_size].sum

pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
    let mut sorted_box_types = box_types.clone();
    sorted_box_types.sort_by(|a, b| b[1].cmp(&a[1]));
    let tmp = sorted_box_types.iter().map(|e|vec![e[1]; e[0] as usize]).collect::<Vec<Vec<i32>>>().into_iter().flatten().collect::<Vec<i32>>();
    let idx = vec![tmp.len(), truck_size as usize].into_iter().min().unwrap();
    tmp[..idx].iter().sum::<i32>()
}

fn main() {
    let box_types = [[1,3],[5,5],[2,5],[4,2],[4,1],[3,1],[2,2],[1,3],[2,5],[3,2]].map(|e|e.to_vec()).to_vec();
    let truck_size = 35;
    println!("{:?}", maximum_units(box_types, truck_size));
}