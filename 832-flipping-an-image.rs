// 示例 1：

// 输入：[[1,1,0],[1,0,1],[0,0,0]]
// 输出：[[1,0,0],[0,1,0],[1,1,1]]
// 解释：首先翻转每一行: [[0,1,1],[1,0,1],[0,0,0]]；
//      然后反转图片: [[1,0,0],[0,1,0],[1,1,1]]

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/flipping-an-image
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
     let image = vec!(
          vec!(1,1,0),
          vec!(1,0,1),
          vec!(0,0,0)
     );
     let res = flip_and_invert_image(image);
     println!("{:?}", res);
}

pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
   image.into_iter().map(|row|row.into_iter().rev().map(|pixel|pixel ^ 1).collect()).collect()
} // into_iter map collect 3人成组出现
