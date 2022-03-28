// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode {
//             next: None,
//             val
//         }
//     }
// }

pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
    let mut current = head;
    let mut arr = vec!();
    let mut ret = 0;
    loop {
        let v = current.as_ref().unwrap().val;
        arr.push(v);
        if current.as_ref().unwrap().next == None {
            break;
        } else {
            current = current.unwrap().next;
        }
    }
    for i in 0..arr.len()/2 {
        let tmp = arr[i] + arr[arr.len()-i-1];
        if tmp > ret {
            ret = tmp;
        }
    }
    ret
}

fn main() {
    let listnode3 = Box::new(ListNode {
        val: 1,
        next: None
    });
    let listnode4 = Box::new(ListNode {
        val: 2,
        next: Some(listnode3)
    });
    let listnode5 = Box::new(ListNode {
        val: 4,
        next: Some(listnode4)
    });
    let listnode6 = Box::new(ListNode {
        val: 5,
        next: Some(listnode5)
    });
    let head = Some(listnode6);
    println!("{:?}", pair_sum(head));
}
