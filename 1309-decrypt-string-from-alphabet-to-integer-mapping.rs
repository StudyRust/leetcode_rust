// 示例 1：

// 输入：s = "10#11#12"
// 输出："jkab"
// 解释："j" -> "10#" , "k" -> "11#" , "a" -> "1" , "b" -> "2".

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/decrypt-string-from-alphabet-to-integer-mapping
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

pub fn freq_alphabets(s: String) -> String {
    s.replace("10#", "j")
     .replace("11#", "k")
     .replace("12#", "l")
     .replace("13#", "m")
     .replace("14#", "n")
     .replace("15#", "o")
     .replace("16#", "p")
     .replace("17#", "q")
     .replace("18#", "r")
     .replace("19#", "s")
     .replace("20#", "t")
     .replace("21#", "u")
     .replace("22#", "v")
     .replace("23#", "w")
     .replace("24#", "x")
     .replace("25#", "y")
     .replace("26#", "z")
     .replace("1", "a")
     .replace("2", "b")
     .replace("3", "c")
     .replace("4", "d")
     .replace("5", "e")
     .replace("6", "f")
     .replace("7", "g")
     .replace("8", "h")
     .replace("9", "i")
}

fn main() {
    println!("{:?}", freq_alphabets("1234".to_string()));
}
