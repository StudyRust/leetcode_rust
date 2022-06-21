struct MinStack {
    stk: Vec<i32>
}

impl MinStack {

    fn new() -> Self {
        MinStack {
            stk: vec!()
        }
    }

    fn push(&mut self, val: i32) {
        self.stk.push(val);
    }

    fn pop(&mut self) {
        self.stk.pop();
    }

    fn top(&self) -> i32 {
        *self.stk.iter().last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.stk.iter().min().unwrap()
    }
}

fn main() {
    let mut obj = MinStack::new();
    obj.push(-2);
    obj.push(0);
    obj.push(-3);
    println!("{:?}", obj.get_min());
    println!("{:?}", obj.top());
    obj.pop();
    println!("{:?}", obj.get_min());
    println!("{:?}", obj.top());
}
