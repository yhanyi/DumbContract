use crate::user::User;

pub struct Transaction {
    pub from: String,
    pub amount: u64,
}

impl Transaction {
    pub fn new(user: &User, amount: u64) -> Self {
        Transaction {
            from: user.name.clone(),
            amount,
        }
    }
}
