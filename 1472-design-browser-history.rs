struct BrowserHistory {
    his: Vec::<String>,
    idx: usize,
}

impl BrowserHistory {

    fn new(homepage: String) -> Self {
        BrowserHistory {
            his: vec![homepage.clone()],
            idx: 0
        }
    }

    fn visit(&mut self, url: String) {
        self.his.truncate(self.idx + 1);
        self.his.push(url.clone());
        self.idx += 1;
    }

    fn back(&mut self, steps: i32) -> String {
        let steps = steps as usize;
        if self.idx < steps {
            self.idx = 0;
        } else {
            self.idx -= steps;
        }
        // println!("{:?} {:?}", self.his, self.idx);
        self.his[self.idx].clone()
    }

    fn forward(&mut self, steps: i32) -> String {
        let steps = steps as usize;
        if self.idx + steps < self.his.len() {
            self.idx += steps;
        } else {
            self.idx = self.his.len() - 1;
        }
        // println!("{:?} {:?}", self.his, self.idx);
        self.his[self.idx].clone()
    }
}

fn main() {
    let mut obj = BrowserHistory::new("leetcode.com".to_string());
    obj.visit("google.com".to_string());
    obj.visit("facebook.com".to_string());
    obj.visit("youtube.com".to_string());
    println!("{:?}", obj.back(1));
    println!("{:?}", obj.back(1));
    println!("{:?}", obj.forward(1));
    obj.visit("linkedin.com".to_string());
    println!("{:?}", obj.forward(2)); // linkedin
    println!("{:?}", obj.back(2));
    println!("{:?}", obj.back(7));
}
