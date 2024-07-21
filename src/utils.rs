use crate::user::User;
use crate::transaction::Transaction;
use crate::condition::Condition;
use rand::Rng;

pub fn generate_users(count: usize) -> Vec<User> {
    (0..count)
        .map(|i| User::new(&format!("User{}", i + 1), rand::thread_rng().gen_range(100..1000)))
        .collect()
}

pub fn random_transaction(user: &User) -> Transaction {
    Transaction::new(user, rand::thread_rng().gen_range(50..200))
}

pub fn random_condition() -> Condition {
    let conditions = vec!["Delivery", "Service", "Product"];
    let description = conditions[rand::thread_rng().gen_range(0..conditions.len())];
    Condition::new(description)
}
