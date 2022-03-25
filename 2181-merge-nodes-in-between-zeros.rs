// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut current = head;
    let mut arr = vec!();
    let mut tmp = 0;
    loop {
        let v = current.as_ref().unwrap().val;
        if v == 0 && tmp != 0 {
            arr.push(tmp);
            tmp = 0
        } else {
            tmp += v;
        }
        if current.as_ref().unwrap().next == None {
            break;
        } else {
            current = current.unwrap().next;
        }
    }

    let mut ret = Some(Box::new(ListNode {
        val: arr[arr.len()-1],
        next: None
    }));
    for i in (0..(arr.len()-1)).rev() {
        let tmp = ret;
        ret = Some(Box::new(ListNode {
            val: arr[i],
            next: tmp
        }));
    }
    ret
}

fn main() {
    let listnode_1 = Box::new(ListNode {
        val: 0,
        next: None
    });
    let listnode0 = Box::new(ListNode {
        val: 3,
        next: Some(listnode_1)
    });
    let listnode1 = Box::new(ListNode {
        val: 0,
        next: Some(listnode0)
    });
    let listnode2 = Box::new(ListNode {
        val: 2,
        next: Some(listnode1)
    });
    let listnode3 = Box::new(ListNode {
        val: 4,
        next: Some(listnode2)
    });
    let listnode4 = Box::new(ListNode {
        val: 0,
        next: Some(listnode3)
    });
    let listnode5 = Box::new(ListNode {
        val: 1,
        next: Some(listnode4)
    });
    let listnode6 = Box::new(ListNode {
        val: 0,
        next: Some(listnode5)
    });
    let head = Some(listnode6);
    println!("{:?}", merge_nodes(head));
}