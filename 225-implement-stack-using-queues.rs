struct MyStack {
    stack_box: Vec<i32>
}

impl MyStack {
    fn new() -> Self {
        MyStack {
            stack_box: vec!()
        }
    }

    fn push(&mut self, x: i32) {
        self.stack_box.push(x);
    }

    fn pop(&mut self) -> i32 {
        let index = self.stack_box.len()-1;
        let ret = self.stack_box[index];
        self.stack_box.remove(index);
        ret
    }

    fn top(&self) -> i32 {
        *self.stack_box.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.stack_box.len() == 0
    }
}

fn main() {
    let mut obj = MyStack::new();
    obj.push(1);
    obj.push(2);
    obj.push(3);
    let ret_2: i32 = obj.pop();
    let ret_3: i32 = obj.top();
    let ret_4: bool = obj.empty();
    println!("{:?} {:?} {:?}", ret_2, ret_3, ret_4);
}
