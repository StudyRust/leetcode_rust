struct MyQueue {
    v: Vec<i32>
}

impl MyQueue {

    fn new() -> Self {
        MyQueue {
            v: vec!()
        }
    }

    fn push(&mut self, x: i32) {
        self.v.push(x);
    }

    fn pop(&mut self) -> i32 {
        let ret = self.v[0];
        self.v.remove(0);
        ret
    }

    fn peek(&self) -> i32 {
        self.v[0]
    }

    fn empty(&self) -> bool {
        self.v.len() == 0
    }
}

fn main() {
    let mut obj = MyQueue::new();
    obj.push(1);
    obj.push(2);
    let ret_2: i32 = obj.pop();
    let ret_3: i32 = obj.peek();
    let ret_4: bool = obj.empty();
    println!(" {:?} {:?} {:?}", ret_2, ret_3, ret_4);
}
