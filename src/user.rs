#[derive(Clone)]
pub struct User {
    pub name: String,
    pub balance: u64,
}

impl User {
    pub fn new(name: &str, balance: u64) -> Self {
        User {
            name: String::from(name),
            balance,
        }
    }

    pub fn adjust_balance(&mut self, amount: i64) {
        if amount < 0 && self.balance < (-amount as u64) {
            println!("Insufficient funds for {}", self.name);
        } else {
            self.balance = ((self.balance as i64) + amount) as u64;
            println!("New balance for {}: {}", self.name, self.balance);
        }
    }
}
