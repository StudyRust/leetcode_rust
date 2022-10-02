struct ParkingSystem {
    current: Vec<i32>,
    max: Vec<i32>
}

impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        ParkingSystem {
            max: vec!(big, medium, small),
            current: vec![0; 3]
        }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        let car_type = car_type as usize - 1;
        if self.current.get(car_type).unwrap() + 1 > *self.max.get(car_type).unwrap() {
            false
        } else {
            *self.current.get_mut(car_type).unwrap() += 1;
            true
        }
    }
}

fn main() {
    let mut obj = ParkingSystem::new(1, 1, 0);
    println!("{:?}", obj.add_car(1));
    println!("{:?}", obj.add_car(2));
    println!("{:?}", obj.add_car(3));
    println!("{:?}", obj.add_car(1));
}
