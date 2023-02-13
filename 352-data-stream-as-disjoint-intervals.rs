use std::cmp;
struct SummaryRanges {
    set: std::collections::BTreeSet<(i32, i32)>,
}

impl SummaryRanges {
    
    fn new() -> Self {
        SummaryRanges { 
            set: std::collections::BTreeSet::new(),
        }
    }
    
    fn add_num(&mut self, val: i32) {
        let mut new = (val, val);
        let l = *self.set.range(..(val, val)).last().unwrap_or(&new);
        let r= *self.set.range((val, val)..).next().unwrap_or(&new);
        if l.1 >= val - 1 {
            new = (l.0, cmp::max(val, l.1));
            self.set.remove(&l);
        }
        if r.0 <= val + 1 && new.1 < r.1 {
            new.1 = r.1;
            self.set.remove(&r);
        }
        self.set.insert(new);
    }
    
    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.set.iter().map(|(l, r)| vec![*l, *r]).collect()
    }
}
