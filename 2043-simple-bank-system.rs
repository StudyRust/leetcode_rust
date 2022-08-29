struct Bank {
    balance: Vec<i64>
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

impl Bank {
    fn new(balance: Vec<i64>) -> Self {
        Bank {
            balance: balance
        }
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        if self.balance.get(account1 as usize - 1) == None {
            return false;
        }
        if self.balance[account1 as usize - 1] < money {
            return false;
        }
        if self.balance.get(account2 as usize - 1) == None {
            return false;
        }
        self.balance[account1 as usize - 1] -= money;
        self.balance[account2 as usize - 1] += money;
        true
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        if self.balance.get(account as usize - 1) == None {
            return false;
        }
        self.balance[account as usize - 1] += money;
        true
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        if self.balance.get(account as usize - 1) == None {
            return false;
        }
        if self.balance[account as usize - 1] < money {
            return false;
        }
        self.balance[account as usize - 1] -= money;
        true
    }
}

fn main() {
    let mut bank = Bank::new(vec![10, 100, 20, 50, 30]);
    bank.transfer(1, 2, 10);
    bank.deposit(1, 30);
    bank.withdraw(1, 10);
    println!("{:?}", bank.balance.get(0));
}
