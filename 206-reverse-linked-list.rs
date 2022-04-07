// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head == None || head.as_ref() == None {
        return None;
    }
    let mut current2 = head;
    let mut arr2 = vec!();
    loop {
        let v = current2.as_ref().unwrap().val;
        arr2.push(v);
        if current2.as_ref().unwrap().next == None {
            break;
        } else {
            current2 = current2.unwrap().next;
        }
    }
    let mut ret = Box::new(ListNode {
        val: *arr2.first().unwrap(),
        next: None
    });

    for i in &arr2[1 as usize..] {
        let tmp = ret;
        ret = Box::new(ListNode {
            val: *i,
            next: Some(tmp)
        });
    }
    Some(ret)
}

fn main() {
    let n5 = Box::new(ListNode {
        val: 5,
        next: None
    });
    let n4 = Box::new(ListNode {
        val: 4,
        next: Some(n5)
    });
    let n3 = Box::new(ListNode {
        val: 3,
        next: Some(n4)
    });
    let n2 = Box::new(ListNode {
        val: 2,
        next: Some(n3)
    });
    let n1 = Box::new(ListNode {
        val: 1,
        next: Some(n2)
    });
    let list1 = Box::new(ListNode {
        val: 0,
        next: Some(n1)
    });
    println!("{:?}", reverse_list(Some(list1)));
}
