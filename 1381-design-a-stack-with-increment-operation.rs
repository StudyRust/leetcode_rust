struct CustomStack {
    mmp: Vec<i32>,
    max_size: i32,
}

impl CustomStack {

    fn new(max_size: i32) -> Self {
        CustomStack {
            mmp: vec![],
            max_size: max_size,
        }
    }

    fn push(&mut self, x: i32) {
        if self.mmp.len() < self.max_size as usize { self.mmp.push(x) }
    }

    fn pop(&mut self) -> i32 {
        let len = self.mmp.len();
        if len == 0 { return -1 }
        self.mmp.remove(len - 1)
    }

    fn increment(&mut self, k: i32, val: i32) {
        let index = vec![k as usize, self.mmp.len()].into_iter().min().unwrap();
        for i in 0..index { self.mmp[i] += val }
    }
}

fn main() {
    let mut cus_stack = CustomStack::new(3);
    cus_stack.push(1);
    cus_stack.push(2);
    cus_stack.pop();
    cus_stack.push(2);
    cus_stack.push(3);
    cus_stack.push(4);
    cus_stack.increment(5, 100);
    cus_stack.increment(2, 100);
    cus_stack.pop();
    cus_stack.pop();
    cus_stack.pop();
    cus_stack.pop();
}
