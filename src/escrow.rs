use std::collections::HashMap;
use crate::transaction::Transaction;
use crate::condition::Condition;
use crate::user::User;

pub struct Escrow {
    balance: u64,
    condition_met: bool,
    conditions: Vec<Condition>,
    transactions: Vec<Transaction>,
    participants: HashMap<String, User>,
}

impl Escrow {
    pub fn new() -> Self {
        Escrow {
            balance: 0,
            condition_met: false,
            conditions: Vec::new(),
            transactions: Vec::new(),
            participants: HashMap::new(),
        }
    }

    pub fn add_participant(&mut self, user: User) {
        self.participants.insert(user.name.clone(), user);
    }

    pub fn deposit(&mut self, transaction: Transaction) {
        let temp = transaction.amount.clone();
        self.balance += transaction.amount;
        self.transactions.push(transaction);
        println!("Deposited: {}. New balance: {}", temp, self.balance);
    }

    pub fn meet_condition(&mut self, condition: Condition) {
        self.condition_met = true;
        println!("Condition met.");
        self.conditions.push(condition);
    }

    pub fn release_funds(&mut self, user: &User) -> Result<u64, &'static str> {
        if self.condition_met && self.balance > 0 {
            let released_amount = self.balance;
            self.balance = 0;
            println!("Funds released: {} to {}", released_amount, user.name);
            Ok(released_amount)
        } else if !self.condition_met {
            println!("Failed to release funds: Condition not met.");
            Err("Condition not met.")
        } else {
            println!("Failed to release funds: No funds available.");
            Err("No funds available.")
        }
    }
}
