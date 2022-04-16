// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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
    arr.sort_by(|a, b|b.cmp(a));
    let mut ret = Box::new(ListNode {
        val: *arr.first().unwrap(),
        next: None
    });
    for i in &arr[1..] {
        let tmp = ret;
        ret = Box::new(ListNode {
            val: *i,
            next: Some(tmp)
        });
    }
    Some(ret)
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
        val: 2,
        next: Some(n1)
    });
    println!("{:?}", insertion_sort_list(Some(n0)));
}
