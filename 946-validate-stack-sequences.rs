pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
    let mut stack: Vec<i32> = vec![];
    let mut pushed = pushed;
    let mut popped = popped;
    while pushed.len() > 0 {
        let tmp = pushed.remove(0);
        stack.push(tmp);
        if tmp == popped[0] {
            // 这里应该持续出栈
            while stack.iter().last().is_some() && *stack.iter().last().unwrap() == popped[0] {
                stack.pop();
                popped.remove(0);
            }
        }
    }
    if stack.len() == 0 { return true }
    stack.reverse();
    stack == popped
}

fn main() {
    println!("{:?}", validate_stack_sequences(vec![1,2,3,4,5], vec![4,5,3,2,1]));
    println!("{:?}", validate_stack_sequences(vec![2,1,0], vec![1,2,0]));
}
