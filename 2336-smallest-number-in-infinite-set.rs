struct SmallestInfiniteSet {
    set: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SmallestInfiniteSet {

    fn new() -> Self {
        SmallestInfiniteSet {
            set: (1..=1000).collect()
        }
    }

    fn pop_smallest(&mut self) -> i32 {
        let mut ret = -1;
        for (i, num) in self.set.iter().enumerate() {
            if *num != 0 {
                ret = *num;
                self.set[i] = 0;
                break;
            }
        }
        ret
    }

    fn add_back(&mut self, num: i32) {
        self.set[num as usize - 1] = num;
    }
}

fn main() {
    let mut obj = SmallestInfiniteSet::new();
    obj.add_back(2);
    println!("{:?}", obj.pop_smallest());
    obj.add_back(2);
    println!("{:?}", obj.pop_smallest());
    obj.add_back(1);
    println!("{:?}", obj.pop_smallest());
}