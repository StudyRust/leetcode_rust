// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
    let mut current = head;
    let mut arr = vec!();
    loop {
        let v = current.as_ref().unwrap().val;
        arr.push(v);
        if current.as_ref().unwrap().next == None {
            break;
        } else {
            current = current.unwrap().next;
        }
    }
    let bin_str = arr.iter().map(|e|e.to_string()).collect::<Vec<String>>().join("");
    isize::from_str_radix(&bin_str, 2).unwrap() as i32
}

fn main() {
    let n2 = Box::new(ListNode {
        val: 1,
        next: None
    });
    let n1 = Box::new(ListNode {
        val: 0,
        next: Some(n2)
    });
    let n0 = Box::new(ListNode {
        val: 1,
        next: Some(n1)
    });
    println!("{:?}", get_decimal_value(Some(n0)));
}
