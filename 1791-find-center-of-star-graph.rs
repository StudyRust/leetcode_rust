// 链接：https://leetcode-cn.com/problems/find-center-of-star-graph
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。输入：edges = [[1,2],[2,3],[4,2]]
// 输出：2
// 解释：如上图所示，节点 2 与其他每个节点都相连，所以节点 2 是中心节点。

use std::collections::HashMap;

fn main() {
    let edges = vec!(
        vec!(1,2),
        vec!(2,3),
        vec!(4,2)
    );
    println!("{:?}", find_center(edges));
}

pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
    let mut e_map = HashMap::new();
    let mut max = 0;
    let mut current_e = 0;
    for point in edges {
        for e in point {
            if !e_map.contains_key(&e) {
                e_map.insert(e, 1);
            } else {
                *e_map.get_mut(&e).unwrap() += 1;
            }
            if e_map[&e] >= max {
                max = e_map[&e];
                current_e = e;
            }
        }
    }
    current_e
}
