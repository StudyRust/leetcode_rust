// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

pub fn merge_in_between(list1: Option<Box<ListNode>>, a: i32, b: i32, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut current1 = list1;
    let mut arr1 = vec!();
    loop {
        let v = current1.as_ref().unwrap().val;
        arr1.push(v);
        if current1.as_ref().unwrap().next == None {
            break;
        } else {
            current1 = current1.unwrap().next;
        }
    }
    let mut current2 = list2;
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
    let mut ret_arr = vec!();
    let mut front = arr1[0 as usize..a as usize].to_vec();
    ret_arr.append(&mut front);
    ret_arr.append(&mut arr2);
    let mut end = arr1[(b+1) as usize..].to_vec();
    ret_arr.append(&mut end);
    println!("{:?}", ret_arr);
    let mut ret = Box::new(ListNode {
        val: *ret_arr.last().unwrap(),
        next: None
    });

    for i in ret_arr[0..ret_arr.len()-1].iter().rev() {
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
    let a = 3;
    let b = 4;
    let n2 = Box::new(ListNode {
        val: 1000002,
        next: None
    });
    let n1 = Box::new(ListNode {
        val: 1000001,
        next: Some(n2)
    });
    let list2 = Box::new(ListNode {
        val: 1000000,
        next: Some(n1)
    });
    println!("{:?}", merge_in_between(Some(list1), a, b, Some(list2)));
}
